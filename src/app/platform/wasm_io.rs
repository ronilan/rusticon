use std::sync::{Arc, LazyLock, Mutex, Once};

use crate::{
    core::{
        io::RusticonIo,
        model::AppPhase,
        shared::{ImportOutcome, RESULT_HOLDER},
    },
    features::{export::build_svg, import_payload::import_payload_svg, message::draw_message},
    State,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::DragEvent;

#[derive(Clone, Default)]
pub struct WasmIo;

#[derive(Default)]
struct LaunchState {
    drop_ready: bool,
    pending_outcome: Option<ImportOutcome>,
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
    let Some(document) = window.document() else {
        return;
    };
    let Some(surface) = document.get_element_by_id("surface") else {
        return;
    };

    let prevent_default = Closure::<dyn FnMut(_)>::new(move |event: DragEvent| {
        event.prevent_default();
    });
    let _ = surface
        .add_event_listener_with_callback("dragover", prevent_default.as_ref().unchecked_ref());
    prevent_default.forget();

    let on_drop = Closure::<dyn FnMut(_)>::new(move |event: DragEvent| {
        event.prevent_default();

        let Some(data_transfer) = event.data_transfer() else {
            return;
        };
        let Some(files) = data_transfer.files() else {
            return;
        };
        let Some(file) = files.get(0) else {
            return;
        };

        let file_name = file.name();
        let promise = file.text();

        spawn_local(async move {
            let outcome = match JsFuture::from(promise).await {
                Ok(js_value) => {
                    if let Some(text) = js_value.as_string() {
                        import_payload_svg(&file_name, &text)
                    } else {
                        Err("Failed to read dropped file text.".to_string())
                    }
                }
                Err(_) => Err("Failed to read dropped file.".to_string()),
            };

            let mut launch = LAUNCH_STATE.lock().unwrap();
            launch.pending_outcome = Some(outcome);
            launch.drop_ready = true;
        });
    });

    let _ = surface.add_event_listener_with_callback("drop", on_drop.as_ref().unchecked_ref());
    on_drop.forget();
}

impl RusticonIo for WasmIo {
    fn initial_file_path(&self) -> String {
        "favicon.svg".to_string()
    }

    fn initial_phase(&self) -> AppPhase {
        AppPhase::Launch
    }

    fn start_import(&self, path: String) {
        let mut guard = RESULT_HOLDER.lock().unwrap();
        let mut launch = LAUNCH_STATE.lock().unwrap();

        if launch.drop_ready {
            *guard = launch.pending_outcome.take();
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
        let file_name = self.normalize_svg_name(&final_ui_state.editor.file_path);

        match self.download_svg(&svg, &file_name) {
            Ok(_) => self.report_message("Export download started.", 10),
            Err(err_msg) => self.report_message(&err_msg, 196),
        }
    }
}
