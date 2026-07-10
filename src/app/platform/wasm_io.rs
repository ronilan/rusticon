use std::sync::{Arc, LazyLock, Mutex, Once};

use crate::{
    core::{
        io::RusticonIo,
        shared::{ImportOutcome, RESULT_HOLDER},
    },
    features::{export::build_svg, import::import_bytes, message::draw_message},
    platform::FileHandle,
    State,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{DragEvent, FileSystemFileHandle, FileSystemWritableFileStream};

#[derive(Clone, Default)]
pub struct WasmIo;

#[derive(Default)]
struct LaunchState {
    drop_ready: bool,
    /// Handle captured from a drag-and-drop. `None` when the dropped file was
    /// not already an SVG/Crumbicon (its path was rewritten to `.svg`), so a
    /// save must fall back to the Save-As prompt instead of overwriting it.
    pending_handle: Option<JsValue>,
}

static LAUNCH_STATE: LazyLock<Arc<Mutex<LaunchState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(LaunchState::default())));
static LISTENERS_ONCE: Once = Once::new();

impl WasmIo {
    pub fn new() -> Self {
        LISTENERS_ONCE.call_once(setup_drop_listeners);
        Self
    }

    fn query_param(&self, name: &str) -> Option<String> {
        let window = web_sys::window()?;
        let location = window.location();
        let hash = location.hash().ok()?;
        let hash = hash.strip_prefix("#").unwrap_or(&hash);
        let params = web_sys::UrlSearchParams::new_with_str(hash).ok()?;
        params.get(name)
    }

    fn initial_size(&self) -> usize {
        self.query_param("size")
            .and_then(|raw| raw.parse::<usize>().ok())
            .filter(|n| *n == 8 || *n == 16)
            .unwrap_or(8)
    }

    fn normalize_svg_name(&self, file_name: &str) -> String {
        if file_name.to_lowercase().ends_with(".svg") {
            file_name.to_string()
        } else {
            format!("{}.svg", file_name)
        }
    }

    async fn save_to_handle(&self, handle: JsValue, content: String) -> Result<(), JsValue> {
        let handle: FileSystemFileHandle = handle.unchecked_into();
        let writable = JsFuture::from(handle.create_writable()).await?;
        let stream: FileSystemWritableFileStream = writable.unchecked_into();

        JsFuture::from(stream.write_with_str(&content)?).await?;
        JsFuture::from(stream.close()).await?;
        Ok(())
    }

    async fn save_as_wasm(
        &self,
        content: String,
        suggested_name: &str,
    ) -> Result<(JsValue, String), JsValue> {
        let window = web_sys::window().unwrap();
        let picker_fn = js_sys::Reflect::get(&window, &JsValue::from_str("showSaveFilePicker"))?
            .dyn_into::<js_sys::Function>()?;

        // Pre-fill the browser's permission prompt with the corrected `.svg` name.
        let opts = js_sys::Object::new();
        js_sys::Reflect::set(
            &opts,
            &JsValue::from_str("suggestedName"),
            &JsValue::from_str(suggested_name),
        )?;

        let promise = picker_fn.call1(&window, &opts)?;
        let handle_js = JsFuture::from(promise.unchecked_into::<js_sys::Promise>()).await?;
        let handle: FileSystemFileHandle = handle_js.clone().unchecked_into();
        let name = handle.name();

        self.save_to_handle(handle_js.clone(), content).await?;
        Ok((handle_js, name))
    }

    fn download_svg(&self, svg: &str, file_name: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or_else(|| "window not available".to_string())?;
        let document = window
            .document()
            .ok_or_else(|| "document not available".to_string())?;
        let body = document
            .body()
            .ok_or_else(|| "document.body not available".to_string())?;

        let parts = js_sys::Array::new();
        parts.push(&JsValue::from_str(svg));
        let blob = web_sys::Blob::new_with_str_sequence(&parts)
            .map_err(|_| "failed to create blob".to_string())?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)
            .map_err(|_| "failed to create object URL".to_string())?;

        let anchor = document
            .create_element("a")
            .map_err(|_| "failed to create anchor".to_string())?
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .map_err(|_| "failed to cast anchor element".to_string())?;

        anchor.set_href(&url);
        anchor.set_download(file_name);
        let _ = anchor.set_attribute("style", "display:none");

        let _ = body.append_child(&anchor);
        anchor.click();
        anchor.remove();
        let _ = web_sys::Url::revoke_object_url(&url);

        Ok(())
    }
}

fn setup_drop_listeners() {
    let Some(window) = web_sys::window() else {
        return;
    };

    // Prevent default browser behavior on the whole window
    let prevent_default = Closure::<dyn FnMut(_)>::new(move |event: DragEvent| {
        event.prevent_default();
    });
    let _ = window.add_event_listener_with_callback("dragover", prevent_default.as_ref().unchecked_ref());
    prevent_default.forget();

    let on_drop = Closure::<dyn FnMut(_)>::new(move |event: DragEvent| {
        event.prevent_default();

        let Some(data_transfer) = event.data_transfer() else {
            return;
        };

        spawn_local(async move {
            let items = data_transfer.items();
            if items.length() > 0 {
                let item = items.get(0).unwrap();
                if item.kind() == "file" {
                    // Capture handle using Reflect hack (not yet in web-sys types)
                    let handle_promise = js_sys::Reflect::get(&item, &JsValue::from_str("getAsFileSystemHandle"))
                        .unwrap()
                        .dyn_into::<js_sys::Function>()
                        .unwrap()
                        .call0(&item)
                        .unwrap();

                    let handle: JsValue =
                        JsFuture::from(handle_promise.unchecked_into::<js_sys::Promise>())
                            .await
                            .unwrap();
                    let file_handle: FileSystemFileHandle = handle.clone().unchecked_into();
                    let file = JsFuture::from(file_handle.get_file()).await.unwrap();
                    let file: web_sys::File = file.unchecked_into();

                    let file_name = file.name();
                    let buffer = JsFuture::from(file.array_buffer()).await.unwrap();
                    let bytes = js_sys::Uint8Array::new(&buffer).to_vec();

                    let outcome = import_bytes(&file_name, &bytes);

                    // Decide whether the original file handle may be reused on save.
                    // For non-SVG drops the path is deliberately rewritten to `.svg`
                    // inside `import_bytes`, but the handle still points at the
                    // original file. If we kept it, a save would silently overwrite
                    // that file and never show a permission prompt. Clearing the
                    // handle makes Save fall back to the Save-As prompt, which is
                    // pre-filled with the corrected `.svg` name. We only keep the
                    // handle when the file name was unchanged (already SVG/Crumbicon).
                    let keep_handle = match &outcome {
                        Ok((_, _, _, returned_path)) => returned_path.eq_ignore_ascii_case(&file_name),
                        Err(_) => false,
                    };

                    let mut launch = LAUNCH_STATE.lock().unwrap();
                    *RESULT_HOLDER.lock().unwrap() = Some(outcome);
                    launch.pending_handle = if keep_handle { Some(handle) } else { None };
                    launch.drop_ready = true;

                    // The current app (main editor) only terminates when its
                    // `on_set` fires, which is driven by Globals::exit(). Without
                    // this, dropping a file while the editor is open would set
                    // `drop_ready` but the editor would keep running and never
                    // hand off to the next file. Exit here so the running app's
                    // `on_set` detects the drop and re-runs the flow.
                    incredible::exit();
                }
            }
        });
    });

    let _ = window.add_event_listener_with_callback("drop", on_drop.as_ref().unchecked_ref());
    on_drop.forget();
}

impl RusticonIo for WasmIo {
    fn initial_file_path(&self) -> String {
        self.query_param("name")
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| "favicon.svg".to_string())
    }

    fn reset_import_result(&self) {
        let mut guard = RESULT_HOLDER.lock().unwrap();
        *guard = None;
    }

    fn load_file_in_background(&self, path: String) {
        // Browser mode starts from an empty canvas by default.
        // Hash params control bootstrap metadata:
        // - #name=icon.svg sets the export file name (default favicon.svg)
        // - #size=8 or #size=16 (default 8)
        let size = self.initial_size();
        let outcome: ImportOutcome = Ok((
            vec![None; size * size],
            vec![None; 8],
            size as u8,
            self.normalize_svg_name(&path),
        ));

        let mut guard = RESULT_HOLDER.lock().unwrap();
        *guard = Some(outcome);
    }

    fn take_import_result(&self) -> Option<ImportOutcome> {
        RESULT_HOLDER.lock().unwrap().take()
    }

    fn launch_drop_ready(&self) -> bool {
        LAUNCH_STATE.lock().unwrap().drop_ready
    }

    fn take_pending_handle(&self) -> Option<FileHandle> {
        LAUNCH_STATE.lock().unwrap().pending_handle.take()
    }

    fn consume_drop(&self) {
        let mut launch = LAUNCH_STATE.lock().unwrap();
        launch.drop_ready = false;
        launch.pending_handle = None;
    }

    fn report_message(&self, msg: &str, color_code: u8) {
        draw_message(msg, color_code);
    }

    fn handle_final_save(&self, final_ui_state: &State) {
        if !final_ui_state.save_flag {
            return;
        }

        let (data, size) = if final_ui_state.size == 16 {
            (final_ui_state.canvas16_data.clone(), 16)
        } else {
            (final_ui_state.canvas8_data.clone(), 8)
        };

        let svg = build_svg(&data, &final_ui_state.palette_colors, size, size, 32);
        let suggested_name = self.normalize_svg_name(&final_ui_state.file_path);

        let io = self.clone();
        let handle = final_ui_state.file_handle.clone();

        spawn_local(async move {
            if let Some(h) = handle {
                // Overwrite the exact file that was opened.
                match io.save_to_handle(h, svg).await {
                    Ok(_) => io.report_message("Saved successfully.", 10),
                    Err(_) => io.report_message("Save failed.", 196),
                }
            } else {
                // No handle → ask the user where to save (pre-filled with `.svg`).
                match io.save_as_wasm(svg.clone(), &suggested_name).await {
                    Ok(_) => io.report_message("File created.", 10),
                    Err(_) => {
                        // Fallback for browsers without the File System Access API.
                        if io.download_svg(&svg, &suggested_name).is_err() {
                            io.report_message("Save cancelled.", 196);
                        } else {
                            io.report_message("Export download started.", 10);
                        }
                    }
                }
            }
        });
    }

    fn finish_with_error(&self, msg: &str, color_code: u8) {
        self.report_message(msg, color_code);
    }
}
