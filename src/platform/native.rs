use std::{env, thread};

use crate::{
    core::{
        io::RusticonIo,
        model::{AppPhase, State},
        shared::{ImportOutcome, RESULT_HOLDER},
    },
    features::{export::export_svg, import::import_file, message::draw_message},
};
use incredible_elements_extra::{DroppedItem, DroppedSource};

pub type FileHandle = String;

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

    fn initial_phase(&self) -> AppPhase {
        AppPhase::Splash
    }

    fn start_import(&self, path: String) {
        {
            let mut guard = RESULT_HOLDER.lock().unwrap();
            *guard = None;
        }

        let result_holder_thread = RESULT_HOLDER.clone();

        thread::spawn(move || {
            let result = std::panic::catch_unwind(|| import_file(&path))
                .map_err(|e| format!("Panic in import_file: {:?}", e))
                .and_then(|res| res);

            let mut guard = result_holder_thread.lock().unwrap();
            *guard = Some(result);
        });
    }

    fn start_import_drop(&self, item: DroppedItem) {
        if let DroppedSource::Path(path) = &item.source {
            self.start_import(path.to_string_lossy().into_owned());
        }
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

        if let Err(err_msg) = export_svg(
            &data,
            &state.editor.palette_colors,
            size,
            size,
            32,
            &state.editor.file_path,
        ) {
            self.report_message(&err_msg, 196);
        }
    }
}
