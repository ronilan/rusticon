use crate::AppState;
use little_tui::Element;

pub fn build() -> Element<AppState> {
    let wrapper = Element::default();

    wrapper.internals.push(crate::ui::screen::build());
    wrapper.internals.push(crate::ui::centered_modal::build());
    wrapper.internals.push(crate::ui::title_bar::build());

    wrapper.internals.push(crate::ui::canvas_16::build());
    wrapper.internals.push(crate::ui::canvas_8::build());

    wrapper.internals.push(crate::ui::color_picker_16::build());
    wrapper.internals.push(crate::ui::color_picker_216::build());
    wrapper
        .internals
        .push(crate::ui::color_picker_gray::build());
    wrapper
        .internals
        .push(crate::ui::color_picker_empty::build());
    wrapper
        .internals
        .push(crate::ui::color_picker_palette::build());

    wrapper
        .internals
        .push(crate::ui::label_color_selected::build());
    wrapper.internals.push(crate::ui::color_selected::build());
    wrapper.internals.push(crate::ui::color_candidate::build());
    wrapper
        .internals
        .push(crate::ui::label_color_candidate::build());

    wrapper.internals.push(crate::ui::label_new::build());
    wrapper.internals.push(crate::ui::button_8::build());
    wrapper.internals.push(crate::ui::button_16::build());

    wrapper.internals.push(crate::ui::label_end::build());
    wrapper.internals.push(crate::ui::button_save::build());
    wrapper.internals.push(crate::ui::button_exit::build());

    wrapper
}
