use crate::{core::model::AppPhase, core::model::State};
use incredible::*;
use incredible_elements_text_fonts::FigletStr;
use incredible_helpers_effects::{GradientDirection, gradient_color};

// Gradient decorate function for splash logo
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
        [255, 0, 0],
        [0, 255, 0],
    ];
    let look = gradient_color(&stops, GradientDirection::Vertical, &flattened, progress);
    el.look(look);

    // Resolve and store the current decoration based on element style and status.
    el.decoration()
        .active_decor
        .replace(el.decoration().style.resolve(el.status()));
}

pub fn build() -> FigletStr<State> {
    let logo = FigletStr::default();
    logo.text("Rusticon")
        // Set up gradient decoration
        .draw_override(Some(DrawOverride {
            auto_render: None,
            flatten_override: true,
        }));
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

    logo
}
