use crate::core::{
    model::{AppPhase, State},
    shared::ImportOutcome,
};

pub trait RusticonIo {
    fn initial_file_path(&self) -> String;
    fn initial_phase(&self) -> AppPhase {
        AppPhase::Splash
    }
    fn return_to_launch_on_exit(&self) -> bool {
        false
    }
    fn start_import(&self, path: String);
    fn launch_drop_ready(&self) -> bool {
        false
    }
    fn take_import_result(&self) -> Option<ImportOutcome>;
    fn report_message(&self, msg: &str, color_code: u8);
    fn handle_final_save(&self, final_ui_state: &State);
}
