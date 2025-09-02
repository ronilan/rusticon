use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Elements<'a, AppState> {
    let elements: Elements<AppState> = Elements::new();

    elements.push(crate::ui::screen::build());
    elements.push(crate::ui::centered_modal::build());
    elements.push(crate::ui::title_bar::build());

    elements.push(crate::ui::canvas_16::build());
    elements.push(crate::ui::canvas_8::build());

    elements.push(crate::ui::color_picker_16::build());
    elements.push(crate::ui::color_picker_216::build());
    elements.push(crate::ui::color_picker_gray::build());
    elements.push(crate::ui::color_picker_empty::build());
    elements.push(crate::ui::color_picker_palette::build());

    elements.push(crate::ui::label_color_selected::build());
    elements.push(crate::ui::color_selected::build());
    elements.push(crate::ui::color_candidate::build());
    elements.push(crate::ui::label_color_candidate::build());

    elements.push(crate::ui::label_new::build());
    elements.push(crate::ui::button_8::build());
    elements.push(crate::ui::button_16::build());

    elements.push(crate::ui::label_end::build());
    elements.push(crate::ui::button_save::build());
    elements.push(crate::ui::button_exit::build());

    elements
}
