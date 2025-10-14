use crate::SplashState;
use little_tui::Internals;

pub fn build() -> Internals<SplashState> {
    let internals: Internals<SplashState> = Internals::new();

    internals.push(crate::ui::splash_logo::build());
    internals.push(crate::ui::splash_footer::build());

    internals
}
