use crate::SplashState;
use little_tui::Element;

pub fn build() -> Element<SplashState> {
    let wrapper = Element::new();

    wrapper.elements.push(crate::ui::splash_logo::build());
    wrapper.elements.push(crate::ui::splash_footer::build());

    wrapper
}
