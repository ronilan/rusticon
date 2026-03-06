use crate::State;
use little_tui::Element;

pub fn build() -> Element<State> {
    let wrapper = Element::new();

    wrapper.elements.push(crate::ui::screen::build());
    wrapper.elements.push(crate::ui::centered_modal::build());
    wrapper.elements.push(crate::ui::title_bar::build());

    wrapper.elements.push(crate::ui::canvas_16::build());
    wrapper.elements.push(crate::ui::canvas_8::build());

    wrapper.elements.push(crate::ui::color_picker_16::build());
    wrapper.elements.push(crate::ui::color_picker_216::build());
    wrapper.elements.push(crate::ui::color_picker_gray::build());
    wrapper
        .elements
        .push(crate::ui::color_picker_empty::build());
    wrapper
        .elements
        .push(crate::ui::color_picker_palette::build());

    wrapper
        .elements
        .push(crate::ui::label_color_selected::build());
    wrapper.elements.push(crate::ui::color_selected::build());
    wrapper.elements.push(crate::ui::color_candidate::build());
    wrapper
        .elements
        .push(crate::ui::label_color_candidate::build());

    wrapper.elements.push(crate::ui::label_new::build());
    wrapper.elements.push(crate::ui::button_8::build());
    wrapper.elements.push(crate::ui::button_16::build());

    wrapper.elements.push(crate::ui::label_end::build());
    wrapper.elements.push(crate::ui::button_save::build());
    wrapper.elements.push(crate::ui::button_exit::build());

    wrapper
}
