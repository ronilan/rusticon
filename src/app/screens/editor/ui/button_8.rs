use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 67;
static Y: isize = 2;

pub fn build() -> Element<State> {
    let button_8: Element<State> = Element::new();
    button_8
        .x(X)
        .y(Y)
        .look(Look::from("8x8"))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.size = 8;
                state.canvas8_data = vec![None; 64];

                // erase the 16x16 area
                // canvas 16 position
                static EX: isize = 23;
                static EY: isize = 3;
                let eraser: Element<State> = Element::new();
                eraser.x(EX).y(EY).look(Look::from((32, 16, ' ')));
                reposition(&eraser, EX, EY, state);
                // The eraser is "disposable" and not part of the app ui "tree".
                // As such it's depth is set to None and it will not be drawn unless force.
                // Force it.
                draw_forced(&eraser);
            }
        })
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    button_8
}
