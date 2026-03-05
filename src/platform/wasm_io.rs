use crate::{
    core::{
        io::RusticonIo,
        shared::{ImportOutcome, RESULT_HOLDER},
    },
    features::{export::build_svg, message::draw_message},
    State,
};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone, Default)]
pub struct WasmIo;

impl WasmIo {
    pub fn new() -> Self {
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
        let file_name = self.normalize_svg_name(&final_ui_state.file_path);

        match self.download_svg(&svg, &file_name) {
            Ok(_) => self.report_message("Export download started.", 10),
            Err(err_msg) => self.report_message(&err_msg, 196),
        }
    }
}
