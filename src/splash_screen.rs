use crate::SplashState;
use little_tui::Element;

pub fn build() -> Element<SplashState> {
    let wrapper = Element::default();

    wrapper.internals.push(crate::ui::splash_logo::build());
    wrapper.internals.push(crate::ui::splash_footer::build());

    wrapper
}
