use std::sync::{Arc, LazyLock, Mutex};

pub type ImportPayload = (Vec<Option<u8>>, Vec<Option<u8>>, u8, String);
pub type ImportOutcome = Result<ImportPayload, String>;

// Shared between bootstrap orchestration and splash footer.
pub static RESULT_HOLDER: LazyLock<Arc<Mutex<Option<ImportOutcome>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub static DROP_HOLDER: LazyLock<Arc<Mutex<Option<String>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));
