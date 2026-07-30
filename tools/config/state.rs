// -----------------------------
// State
// -----------------------------

#[derive(Clone, PartialEq, Debug, Default)]
pub struct State {
    pub focused_index: usize,
    pub name: String,
    pub app_name: String,
    pub tagline: String,
    pub keywords: String,
    pub description: String,
    pub should_apply: bool,
    pub should_quit: bool,
    pub is_valid: bool,
    pub rule_text: String,
    pub rule_invalid: bool,
}

// -----------------------------
// Validation
// -----------------------------

/// Returns (is_form_valid, rule_hint_for_focused_field, is_field_invalid)
pub fn validate(state: &State) -> (bool, &'static str, bool) {
    // Per-field rules (returned when that field is focused)
    let focused_rule: &'static str = match state.focused_index {
        0 => "Required. Any text, no newlines.",
        1 => "Required. snake_case, ASCII, max 64 chars, starts with a letter.",
        2 => "Optional. No newlines.",
        3 => "Max 5, comma-separated. Each: ASCII, max 20 chars, alphanumeric start.",
        4 => "Plain text, no newlines, max 160 chars recommended.",
        _ => "",
    };

    // --- Name validation ---
    let name_ok = !state.name.is_empty()
        && state.name.len() <= 64
        && state.name.is_ascii()
        && state
            .name
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic())
            .unwrap_or(false)
        && state
            .name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_');

    // --- App Name validation ---
    let app_name_ok = !state.app_name.is_empty();

    // --- Keywords validation ---
    let kw_ok = if state.keywords.is_empty() {
        true
    } else {
        let parts: Vec<&str> = state.keywords.split(',').map(|k| k.trim()).collect();
        parts.len() <= 5
            && parts.iter().all(|k| {
                !k.is_empty()
                    && k.len() <= 20
                    && k.is_ascii()
                    && k.chars()
                        .next()
                        .map(|c| c.is_ascii_alphanumeric())
                        .unwrap_or(false)
                    && k.chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '+')
            })
    };

    // --- Description validation ---
    let desc_ok = state.description.len() <= 160;

    let form_ok = name_ok && app_name_ok && kw_ok && desc_ok;

    // Determine if the *focused* field is currently invalid
    let focused_invalid = match state.focused_index {
        0 => !app_name_ok,
        1 => !name_ok,
        3 => !kw_ok,
        4 => !desc_ok,
        _ => false,
    };

    (form_ok, focused_rule, focused_invalid)
}
