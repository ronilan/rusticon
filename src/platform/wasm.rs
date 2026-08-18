use std::sync::{Arc, LazyLock, Mutex};

use crate::{
    State,
    core::{
        io::RusticonIo,
        model::AppPhase,
        shared::{ImportOutcome, RESULT_HOLDER},
    },
    features::{export::build_svg, import::import_bytes, message::draw_message},
};
use incredible_elements_extra::DroppedItem;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Blob, FileSystemFileHandle, FileSystemWritableFileStream, HtmlAnchorElement, Url};

pub type FileHandle = JsValue;

#[derive(Clone, Default)]
pub struct WasmIo;

#[derive(Default)]
struct LaunchState {
    pending_handle: Option<JsValue>,
    pending_file_path: Option<String>, // Updated file_path from Save As dialog
}

static LAUNCH_STATE: LazyLock<Arc<Mutex<LaunchState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(LaunchState::default())));

impl WasmIo {
    pub fn new() -> Self {
        Self
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

        // Build options with suggestedName
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

    /// Classic <a download> fallback for browsers without showSaveFilePicker
    /// (Firefox, Safari, …).
    fn download_file(&self, content: &str, filename: &str) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or(JsValue::from_str("no window"))?;
        let document = window.document().ok_or(JsValue::from_str("no document"))?;

        // Blob (no options variant — keeps required web-sys features minimal)
        let parts = js_sys::Array::new();
        parts.push(&JsValue::from_str(content));
        let blob = Blob::new_with_str_sequence(&parts)?;

        // Object URL
        let url = Url::create_object_url_with_blob(&blob)?;

        // Hidden <a download>
        let a = document
            .create_element("a")?
            .dyn_into::<HtmlAnchorElement>()?;
        a.set_href(&url);
        a.set_download(filename);
        a.style().set_property("display", "none")?;

        let body = document.body().ok_or(JsValue::from_str("no body"))?;
        body.append_child(&a)?;
        a.click();
        body.remove_child(&a)?;

        Url::revoke_object_url(&url)?;
        Ok(())
    }

    fn has_save_file_picker() -> bool {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return false,
        };
        match js_sys::Reflect::get(&window, &JsValue::from_str("showSaveFilePicker")) {
            Ok(v) => v.is_function(),
            Err(_) => false,
        }
    }
}

impl RusticonIo for WasmIo {
    fn initial_file_path(&self) -> String {
        "favicon.svg".to_string()
    }

    fn initial_phase(&self) -> AppPhase {
        AppPhase::Launch
    }

    fn return_to_launch_on_exit(&self) -> bool {
        true
    }

    fn start_import(&self, path: String) {
        let outcome: ImportOutcome = Ok((
            vec![None; 8 * 8],
            vec![None; 8],
            8,
            self.normalize_svg_name(&path),
        ));
        let mut guard = RESULT_HOLDER.lock().unwrap();
        *guard = Some(outcome);
    }

    fn start_import_drop(&self, item: DroppedItem) {
        let file_name = item.name.clone();
        let file_handle = item.handle.clone();

        let Ok(bytes) = item.read() else {
            return;
        };
        if bytes.is_empty() {
            return;
        }

        let outcome = import_bytes(&file_name, &bytes);

        // Decide whether the original file handle may be reused on save.
        //
        // For non-SVG drops (e.g. `photo.png`) we deliberately rewrite the
        // path to `.svg` inside `import_bytes`, but the handle still points
        // at the original file. If we kept that handle, a Save would
        // silently overwrite `photo.png` with SVG content and never show a
        // permission prompt. Clearing the handle makes Save fall back to the
        // Save As flow, which pre-fills the browser prompt with the corrected
        // `.svg` name (state.editor.file_path). We only keep the handle when
        // the file name was unchanged (already an SVG/Crumbicon).
        let keep_handle = match &outcome {
            Ok((_, _, _, returned_path)) => returned_path.eq_ignore_ascii_case(&file_name),
            Err(_) => false,
        };

        LAUNCH_STATE.lock().unwrap().pending_handle = if keep_handle { file_handle } else { None };
        let mut guard = RESULT_HOLDER.lock().unwrap();
        *guard = Some(outcome);
    }

    fn take_import_result(&self) -> Option<ImportOutcome> {
        RESULT_HOLDER.lock().unwrap().take()
    }

    fn report_message(&self, msg: &str, color_code: u8) {
        draw_message(msg, color_code);
    }

    fn perform_save(&self, state: &State) {
        let (data, size) = if state.editor.size == 16 {
            (state.editor.canvas16_data.clone(), 16)
        } else {
            (state.editor.canvas8_data.clone(), 8)
        };

        let svg = build_svg(&data, &state.editor.palette_colors, size, size, 32);

        let io = self.clone();
        let handle = state.editor.file_handle.clone();
        let suggested_name = state.editor.file_path.clone();

        spawn_local(async move {
            // Firefox / Safari: no showSaveFilePicker → download fallback
            if !WasmIo::has_save_file_picker() {
                match io.download_file(&svg, &suggested_name) {
                    Ok(()) => {
                        // Keep the name so the UI stays consistent; no reusable handle.
                        let mut launch = LAUNCH_STATE.lock().unwrap();
                        launch.pending_file_path = Some(suggested_name);
                        // pending_handle stays None
                    }
                    Err(_) => io.report_message("Save failed.", 196),
                }
                return;
            }

            // Chromium path
            if let Some(h) = handle {
                if let Err(_) = io.save_to_handle(h, svg).await {
                    io.report_message("Save failed.", 196);
                }
            } else {
                // Save As flow
                match io.save_as_wasm(svg, &suggested_name).await {
                    Ok((new_handle, new_name)) => {
                        let mut launch = LAUNCH_STATE.lock().unwrap();
                        launch.pending_handle = Some(new_handle);
                        launch.pending_file_path = Some(new_name);
                    }
                    Err(_) => io.report_message("Save cancelled.", 196),
                }
            }
        });
    }

    fn take_pending_handle(&self) -> Option<crate::platform::FileHandle> {
        LAUNCH_STATE.lock().unwrap().pending_handle.take()
    }

    fn take_pending_file_path(&self) -> Option<String> {
        LAUNCH_STATE.lock().unwrap().pending_file_path.take()
    }
}
