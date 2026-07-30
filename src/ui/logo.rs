use crate::{core::model::AppPhase, core::model::State};
use incredible::*;
use incredible_elements_text_fonts::FigletStr;
use incredible_helpers_effects::{GradientDirection, gradient_color_decorate};

const COLOR_STOPS: [Color; 6] = [
    Color::ansi(1),
    Color::ansi(2),
    Color::ansi(3),
    Color::ansi(4),
    Color::ansi(5),
    Color::ansi(6),
];

fn logo_gradient_color_decorate(el: &FigletStr<State>) {
    gradient_color_decorate(
        el,
        &COLOR_STOPS,
        GradientDirection::Vertical,
        logo_gradient_color_decorate,
    );
}

pub fn build() -> FigletStr<State> {
    let logo = FigletStr::default();
    logo.text("Rusticon")
        // Set up gradient decoration
        .draw_override(Some(DrawOverride {
            auto_render: None,
            flatten_override: true,
        }));
    logo.renderer.decorate.set(logo_gradient_color_decorate);

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
