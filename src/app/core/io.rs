use crate::{core::shared::ImportOutcome, platform::FileHandle, State};

pub trait RusticonIo {
    fn initial_file_path(&self) -> String;
    fn reset_import_result(&self);
    fn load_file_in_background(&self, path: String);
    fn take_import_result(&self) -> Option<ImportOutcome>;
    fn report_message(&self, msg: &str, color_code: u8);
    fn handle_final_save(&self, final_ui_state: &State);
    fn finish_with_error(&self, msg: &str, color_code: u8);

    /// WASM drag-and-drop: true once a file has been dropped and is ready to
    /// replace the current import. Default `false` (native does not drop).
    fn launch_drop_ready(&self) -> bool {
        false
    }

    /// WASM drag-and-drop: take the `FileSystemFileHandle` captured from the
    /// dropped file so it can be reused on save. Default `None`.
    fn take_pending_handle(&self) -> Option<FileHandle> {
        None
    }

    /// WASM drag-and-drop: clear the drop-ready flag after it has been consumed.
    /// Default no-op.
    fn consume_drop(&self) {}
}
