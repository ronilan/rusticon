use crate::{core::shared::ImportOutcome, State};

pub trait RusticonIo {
    fn initial_file_path(&self) -> String;
    fn reset_import_result(&self);
    fn load_file_in_background(&self, path: String);
    fn take_import_result(&self) -> Option<ImportOutcome>;
    fn report_message(&self, msg: &str, color_code: u8);
    fn handle_final_save(&self, final_ui_state: &State);
}
