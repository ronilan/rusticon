use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 3;
static Y: i16 = 2;

pub fn build() -> Element<AppState> {
    let mut color_picker_216: Element<AppState> = Element::new(
        Pos::new(X, Y),
        Look::from(
            (0..18)
                .map(|row| {
                    (0..12)
                        .map(|col| {
                            // original crumb formula: (row * 12) + (col * 16)
                            let code = (row * 12 + col + 16).min(231) as u8;
                            terminal_style::format::background(code, " ").unwrap()
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
        ),
    );

    color_picker_216.listener.on_mouse = Some(Box::new(|el, state, event| {
        if event.kind == "move" || event.kind == "click" {
            let row = event.coords.y.get().saturating_sub(el.pos.y.get()) as u8;
            let col = event.coords.x.get().saturating_sub(el.pos.x.get()) as u8;
            let ansi_code = row * 12 + col + 16;
            state.candidate = Some(ansi_code);

            if event.kind == "move" {
                state.picker_mode = true;
            }
            if event.kind == "click" {
                state.paintbrush = Some(ansi_code);
                set_palette_in_state(state, state.candidate);
            }
        }
    }));
    color_picker_216.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    color_picker_216
}
