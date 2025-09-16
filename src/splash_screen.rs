use crate::SplashState;
use little_tui::engine::BaseElements;

pub fn build<'a>() -> BaseElements<'a, SplashState> {
    let elements: BaseElements<SplashState> = BaseElements::new();

    elements.push(crate::ui::splash_logo::build());
    elements.push(crate::ui::splash_footer::build());

    elements
}
