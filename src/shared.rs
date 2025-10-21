use std::sync::{Arc, LazyLock, Mutex};

// Shared between main() and the splash footer
pub static RESULT_HOLDER: LazyLock<
    Arc<Mutex<Option<Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8, String), String>>>>,
> = LazyLock::new(|| Arc::new(Mutex::new(None)));
