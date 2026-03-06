use std::{env, thread};

use crate::{
    core::{
        io::RusticonIo,
        shared::{ImportOutcome, RESULT_HOLDER},
    },
    features::{export::export_svg, import::import_file, message::draw_message},
    State,
};

#[derive(Clone, Default)]
pub struct NativeIo;

impl NativeIo {
    pub fn new() -> Self {
        Self
    }
}

impl RusticonIo for NativeIo {
    fn initial_file_path(&self) -> String {
        env::args()
            .nth(1)
            .unwrap_or_else(|| "favicon.svg".to_string())
    }

    fn reset_import_result(&self) {
        let mut guard = RESULT_HOLDER.lock().unwrap();
        *guard = None;
    }

    fn load_file_in_background(&self, path: String) {
        let result_holder_thread = RESULT_HOLDER.clone();

        thread::spawn(move || {
            let result = std::panic::catch_unwind(|| import_file(&path))
                .map_err(|e| format!("Panic in import_file: {:?}", e))
                .and_then(|res| res);

            let mut guard = result_holder_thread.lock().unwrap();
            *guard = Some(result);
        });
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

        match export_svg(
            &data,
            &final_ui_state.editor.palette_colors,
            size,
            size,
            32,
            &final_ui_state.editor.file_path,
        ) {
            Ok(_) => self.report_message("Export succeeded!", 10),
            Err(err_msg) => self.report_message(&err_msg, 196),
        }
    }
}
