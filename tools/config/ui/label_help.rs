use incredible::*;
use incredible_elements::Label;

use crate::state::State;

pub fn build_label_help() -> Label<State> {
    let label_help: Label<State> = Label::default();
    label_help
        .x(2)
        .y(19)
        .wrap_at(76)
        .focused(false)
        .interactive(false);

    label_help.on_state(|el, state| {
        let text = match state.focused_index {
            0 => " App Name: Displayed in the OS, window titles, and HTML title.",
            1 => " Name: Used for the Rust crate, binaries, and package.json.",
            2 => " Tagline: Appended to the HTML page title for SEO.",
            3 => " Keywords: Used for crate publishing and HTML SEO meta tags.",
            4 => " Description: Used for crate publishing and HTML SEO meta tags.",
            5 => " Quit: Discard any changes and exit.",
            6 => " Apply: Save all changes to the project files.",
            _ => "",
        };
        el.text(text);
    });
    label_help
}
