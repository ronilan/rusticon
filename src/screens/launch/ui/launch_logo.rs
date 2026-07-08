use crate::{core::model::State, ui::APP_WIDTH};
use incredible::*;
use incredible_elements::{Label, Rectangle};
use incredible_elements_text_fonts::{FigletStr, FontSize};
use incredible_helpers_effects::{GradientDirection, gradient_color};
use incredible_helpers_layout::{Flowers, arrangers::Arrangers};

// Gradient decorate function for launch logo (static)
fn logo_gradient_decorate(el: &FigletStr<State>) {
    // FigletStr composes sub elements.
    // Create a look that is fully composited and decorated, ready for further processing.
    let flattened = flatten_composite(el, decorate, logo_gradient_decorate);

    // Define gradient and create look.
    let stops: [[u8; 3]; 6] = [
        [255, 0, 0],
        [0, 255, 0],
        [0, 0, 255],
        [255, 255, 0],
        [0, 255, 255],
        [255, 0, 255],
    ];
    let look = gradient_color(&stops, GradientDirection::Vertical, &flattened, 0.0);
    el.look(look);

    // Resolve and store the current decoration based on element style and status.
    el.decoration()
        .active_decor
        .replace(el.decoration().style.resolve(el.status()));
}

pub fn build() -> Rectangle<State> {
    let launch_logo = Rectangle::new();
    launch_logo.width(APP_WIDTH).height(8);

    // Build the main FigletStr logo (static)
    let logo = FigletStr::default();
    logo.text("Rusticon").font_size(FontSize::Medium);

    // Set up static gradient decoration
    logo.draw_override(Some(DrawOverride {
        auto_render: None,
        flatten_override: true,
    }));
    logo.renderer.decorate.set(logo_gradient_decorate);

    // Build static subtitle text
    let subtitle = Label::default();
    subtitle
        .text("An icon editor for the terminal (and elsewhere too)")
        .wrap_at(34)
        .faint(true);

    launch_logo.add(logo);
    launch_logo.add(subtitle);

    // Center the logo vertically in the wrapper
    launch_logo.elements_flow_down(0);
    launch_logo.elements_to_center();

    launch_logo
}
