use crate::core::model::State;
use incredible::*;
use incredible_elements::Bouncer;

pub fn build() -> Bouncer<State> {
    let subtitle = Bouncer::default();

    subtitle
        .text("An icon editor for the terminal (and elsewhere too)")
        .wrap_at(35)
        .width(47)
        .height(2)
        .interval(50)
        .faint(true);

    subtitle
}
