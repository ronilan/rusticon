use crate::{core::model::AppPhase, core::model::State, ui::APP_WIDTH};
use incredible::*;
use incredible_elements::{Bouncer, Rectangle};
use incredible_elements_text_fonts::{FigletStr, FontSize};
use incredible_helpers_effects::{GradientDirection, gradient_color};
use incredible_helpers_layout::{Flowers, arrangers::Arrangers};

// Gradient decorate function for splash logo (following pattern from website/main.rs)
fn logo_gradient_decorate(el: &FigletStr<State>) {
    // FigletStr composes sub elements.
    // Create a look that is fully composited and decorated, ready for further processing.
    let flattened = flatten_composite(el, decorate, logo_gradient_decorate);

    // Use animation progress as gradient offset if animation has ticked,
    // otherwise fall back to the last known progress, then 0.0.
    let progress = if let Some(anim) = el.get_animation() {
        if let Some(p) = anim.progress {
            p
        } else if let Some(p) = anim.last_progress {
            p
        } else {
            0.0
        }
    } else {
        0.0
    };

    // Define gradient and create look.
    let stops: [[u8; 3]; 6] = [
        [255, 0, 0],
        [0, 255, 0],
        [0, 0, 255],
        [255, 255, 0],
        [0, 255, 255],
        [255, 0, 255],
    ];
    let look = gradient_color(&stops, GradientDirection::Vertical, &flattened, progress);
    el.look(look);

    // Resolve and store the current decoration based on element style and status.
    el.decoration()
        .active_decor
        .replace(el.decoration().style.resolve(el.status()));
}

pub fn build() -> Rectangle<State> {
    let splash_logo = Rectangle::new();
    splash_logo.width(APP_WIDTH).height(8);

    // Build the main FigletStr logo
    let logo = FigletStr::default();
    logo.text("Rusticon").font_size(FontSize::Medium);

    // Set up gradient decoration with animation (following website/main.rs pattern)
    logo.draw_override(Some(DrawOverride {
        auto_render: None,
        flatten_override: true,
    }));
    logo.animation(Some(Animation::new(1000.0, 8.0, 10.0)));
    logo.renderer.decorate.set(logo_gradient_decorate);

    // Animate gradient on loop - trigger draw when animation has progress
    logo.on_loop(|el, state: &mut State, _event| {
        if state.flow.phase != AppPhase::Splash {
            return;
        }
        if let Some(anim) = el.get_animation() {
            if let Some(_) = anim.progress {
                el.draw();
            }
        }
    });

    // Build bouncing subtitle text
    let subtitle = Bouncer::default();
    subtitle
        .text("An icon editor for the terminal (and elsewhere too)")
        .wrap_at(34)
        .faint(true);

    splash_logo.add(logo);
    splash_logo.add(subtitle);

    // Center the logo vertically in the wrapper
    splash_logo.elements_flow_down(0);
    splash_logo.elements_to_center();

    splash_logo
}
