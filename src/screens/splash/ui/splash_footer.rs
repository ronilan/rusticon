use crate::{core::model::State, ui::APP_HEIGHT};
use incredible::*;
use incredible_elements::Link;

pub fn build() -> Link<State> {
    let splash_footer: Link<State> = Link::default();
    splash_footer
        .y(APP_HEIGHT.saturating_sub(1) as isize)
        .text("Made with Incredible")
        .url("https://www.incredible.rs")
        .faint(true);

    splash_footer
}
