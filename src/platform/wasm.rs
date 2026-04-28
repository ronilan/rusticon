use std::sync::{Arc, LazyLock, Mutex, Once};

use crate::{
    core::{
        io::RusticonIo,
        model::AppPhase,
        shared::{ImportOutcome, RESULT_HOLDER},
    },
    features::{export::build_svg, import::import_bytes, message::draw_message},
    State,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{DragEvent, FileSystemFileHandle, FileSystemWritableFileStream};

pub type FileHandle = JsValue;
pub type DroppedData = (JsValue, Vec<u8>, String); // (handle, bytes, name)

#[derive(Clone, Default)]
pub struct WasmIo;

#[derive(Default)]
struct LaunchState {
    drop_ready: bool,
    pending_outcome: Option<(ImportOutcome, Option<JsValue>)>,
}

static LAUNCH_STATE: LazyLock<Arc<Mutex<LaunchState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(LaunchState::default())));
static LISTENERS_ONCE: Once = Once::new();

impl WasmIo {
    pub fn new() -> Self {
        LISTENERS_ONCE.call_once(setup_drop_listeners);
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

    async fn save_as_wasm(&self, content: String) -> Result<(JsValue, String), JsValue> {
        let window = web_sys::window().unwrap();
        let picker_fn = js_sys::Reflect::get(&window, &JsValue::from_str("showSaveFilePicker"))?
            .dyn_into::<js_sys::Function>()?;

        let promise = picker_fn.call0(&window)?;
        let handle_js = JsFuture::from(promise.unchecked_into::<js_sys::Promise>()).await?;
        let handle: FileSystemFileHandle = handle_js.clone().unchecked_into();
        let name = handle.name();

        self.save_to_handle(handle_js.clone(), content).await?;
        Ok((handle_js, name))
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
    let _ = window
        .add_event_listener_with_callback("dragover", prevent_default.as_ref().unchecked_ref());
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
                    // Capture handle using Reflect hack
                    let handle_promise =
                        js_sys::Reflect::get(&item, &JsValue::from_str("getAsFileSystemHandle"))
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

                    let mut launch = LAUNCH_STATE.lock().unwrap();
                    launch.pending_outcome = Some((outcome, Some(handle)));
                    launch.drop_ready = true;
                }
            }
        });
    });

    let _ = window.add_event_listener_with_callback("drop", on_drop.as_ref().unchecked_ref());
    on_drop.forget();
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
        let mut guard = RESULT_HOLDER.lock().unwrap();
        let mut launch = LAUNCH_STATE.lock().unwrap();

        if launch.drop_ready {
            if let Some((outcome, _handle)) = launch.pending_outcome.take() {
                *guard = Some(outcome);
            }
            launch.drop_ready = false;
            return;
        }

        let outcome: ImportOutcome = Ok((
            vec![None; 8 * 8],
            vec![None; 8],
            8,
            self.normalize_svg_name(&path),
        ));
        *guard = Some(outcome);
    }

    fn launch_drop_ready(&self) -> bool {
        LAUNCH_STATE.lock().unwrap().drop_ready
    }

    fn take_import_result(&self) -> Option<ImportOutcome> {
        RESULT_HOLDER.lock().unwrap().take()
    }

    fn report_message(&self, msg: &str, color_code: u8) {
        draw_message(msg, color_code);
    }

    fn handle_final_save(&self, final_ui_state: &State) {
        if !final_ui_state.editor.save_flag {
            return;
        }

        let (data, size) = if final_ui_state.editor.size == 16 {
            (final_ui_state.editor.canvas16_data.clone(), 16)
        } else {
            (final_ui_state.editor.canvas8_data.clone(), 8)
        };

        let svg = build_svg(&data, &final_ui_state.editor.palette_colors, size, size, 32);

        let io = self.clone();
        let handle = final_ui_state.editor.file_handle.clone();

        spawn_local(async move {
            if let Some(h) = handle {
                match io.save_to_handle(h, svg).await {
                    Ok(_) => io.report_message("Saved successfully.", 10),
                    Err(_) => io.report_message("Save failed.", 196),
                }
            } else {
                // Save As flow
                match io.save_as_wasm(svg).await {
                    Ok((_new_handle, _new_name)) => {
                        io.report_message("File created.", 10);
                        // Future: Push new handle back to State
                    }
                    Err(_) => io.report_message("Save cancelled.", 196),
                }
            }
        });
    }
}
