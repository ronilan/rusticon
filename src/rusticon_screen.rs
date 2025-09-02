use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Elements<'a, AppState> {
    let elements: Elements<AppState> = Elements::new();

    elements.push(crate::elements::screen::build());
    elements.push(crate::elements::centered_modal::build());
    elements.push(crate::elements::title_bar::build());

    elements.push(crate::elements::canvas_16::build());
    elements.push(crate::elements::canvas_8::build());

    elements.push(crate::elements::color_picker_16::build());
    elements.push(crate::elements::color_picker_216::build());
    elements.push(crate::elements::color_picker_gray::build());
    elements.push(crate::elements::color_picker_empty::build());
    elements.push(crate::elements::color_picker_palette::build());

    elements.push(crate::elements::label_color_selected::build());
    elements.push(crate::elements::color_selected::build());
    elements.push(crate::elements::color_candidate::build());
    elements.push(crate::elements::label_color_candidate::build());

    elements.push(crate::elements::label_new::build());
    elements.push(crate::elements::button_8::build());
    elements.push(crate::elements::button_16::build());

    elements.push(crate::elements::label_end::build());
    elements.push(crate::elements::button_save::build());
    elements.push(crate::elements::button_exit::build());

    elements
}
