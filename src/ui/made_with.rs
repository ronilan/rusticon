use crate::{core::model::State, ui::APP_HEIGHT};
use incredible::*;
use incredible_elements::{Label, Link, Rectangle};
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;

pub fn build() -> Rectangle<State> {
    let splash_footer = Rectangle::default();

    let label = Label::default();
    label.text("Made with");

    const COLOR_STOPS: [[u8; 3]; 4] = [[0, 95, 175], [175, 95, 175], [0, 175, 175], [0, 95, 175]];

    fn link_gradient_color_decorate(el: &Link<State>) {
        gradient_color_decorate(
            el,
            &COLOR_STOPS,
            GradientDirection::Horizontal,
            link_gradient_color_decorate,
        );
    }

    let link = Link::default();
    link.pointer_shape(Some(PointerShape::Pointer))
        .text("Incredible.rs")
        .width(13) // TODO: bug in Link
        .url("https://www.incredible.rs")
        .draw_override(Some(DrawOverride {
            auto_render: Some(AutoRenderOptions::default()),
            flatten_override: false,
        }));

    link.renderer.decorate.set(link_gradient_color_decorate);

    splash_footer
        .height(1)
        .width(label.get_width() + link.get_width() + 1)
        .y(APP_HEIGHT.saturating_sub(1) as isize);
    splash_footer.add(label).add(link);
    splash_footer.elements_flow_right(1);

    splash_footer
}
