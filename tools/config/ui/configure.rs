use incredible::*;
use incredible_elements::TextArea;

use crate::state::State;

/// Apply the standard editing key combinations to a `TextArea`:
///
/// - `Ctrl+C` — copy
/// - `Ctrl+X` — cut
/// - `Ctrl+E` — select all
/// - `Ctrl+Z` — undo
/// - `Ctrl+Y` — redo
pub fn shortcuts(textarea: &mut TextArea<State>) {
    textarea
        .copy_combination(Some(KeyCombination::new(Key::Char('c'), &[KeyMod::Ctrl])))
        .cut_combination(Some(KeyCombination::new(Key::Char('x'), &[KeyMod::Ctrl])))
        .select_all_combination(Some(KeyCombination::new(Key::Char('e'), &[KeyMod::Ctrl])))
        .undo_combination(Some(KeyCombination::new(Key::Char('z'), &[KeyMod::Ctrl])))
        .redo_combination(Some(KeyCombination::new(Key::Char('y'), &[KeyMod::Ctrl])));
}
