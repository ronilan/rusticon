use crate::AppState;
use little_tui::Internals;

pub fn build() -> Internals<AppState> {
    let internals: Internals<AppState> = Internals::new();

    internals.push(crate::ui::screen::build());
    internals.push(crate::ui::centered_modal::build());
    internals.push(crate::ui::title_bar::build());

    internals.push(crate::ui::canvas_16::build());
    internals.push(crate::ui::canvas_8::build());

    internals.push(crate::ui::color_picker_16::build());
    internals.push(crate::ui::color_picker_216::build());
    internals.push(crate::ui::color_picker_gray::build());
    internals.push(crate::ui::color_picker_empty::build());
    internals.push(crate::ui::color_picker_palette::build());

    internals.push(crate::ui::label_color_selected::build());
    internals.push(crate::ui::color_selected::build());
    internals.push(crate::ui::color_candidate::build());
    internals.push(crate::ui::label_color_candidate::build());

    internals.push(crate::ui::label_new::build());
    internals.push(crate::ui::button_8::build());
    internals.push(crate::ui::button_16::build());

    internals.push(crate::ui::label_end::build());
    internals.push(crate::ui::button_save::build());
    internals.push(crate::ui::button_exit::build());

    internals
}
