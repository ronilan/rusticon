use incredible::*;
use incredible_elements::Label;

use crate::state::State;

pub fn build_label_heading() -> Label<State> {
    let label_heading: Label<State> = Label::default();
    label_heading.x(2).y(1).text("App Configuration");
    label_heading
}
