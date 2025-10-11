use crate::SplashState;
use little_tui::Elements;

pub fn build() -> Elements<SplashState> {
    let elements: Elements<SplashState> = Elements::new();

    elements.push(crate::ui::splash_logo::build());
    elements.push(crate::ui::splash_footer::build());

    elements
}
