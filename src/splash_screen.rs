use crate::tui_engine::*;
use crate::SplashState;

pub fn build<'a>() -> Elements<'a, SplashState> {
    let elements: Elements<SplashState> = Elements::new();

    elements.push(crate::ui::splash_logo::build());
    elements.push(crate::ui::splash_footer::build());

    elements
}
