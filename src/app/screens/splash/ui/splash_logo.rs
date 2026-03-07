use crate::{core::model::AppPhase, State};
use little_tui::*;

fn bouncing_text(n: usize) -> Vec<Block> {
    let width = 40;
    let text = "An icon editor for the terminal";
    let text_len = text.len();

    let s = if text_len >= width {
        text.chars().take(width).collect::<String>()
    } else {
        let max_pad = width - text_len;
        let cycle = max_pad * 2;
        let pos = n % cycle;
        let pad = if pos <= max_pad { pos } else { cycle - pos };
        let right_pad = width - text_len - pad;

        format!(
            "{:pad$}{}{:right_pad$}",
            "",
            text,
            "",
            pad = pad,
            right_pad = right_pad
        )
    };

    s.chars().map(|c| Block::new(c, Decor::default())).collect()
}

fn art_line(n: usize, s: &str) -> Vec<Block> {
    let color = (n % 5) as u8;

    s.chars()
        .map(|c| {
            let d = Decor::default();
            d.color.replace(Some(color.into()));
            Block::new(c, d)
        })
        .collect()
}

pub fn build() -> Element<State> {
    let splash_logo = Element::new();
    splash_logo.look(Look::from((39, 7)));

    splash_logo.on_loop(|el, state: &mut State, event| {
        if state.flow.phase != AppPhase::Splash && state.flow.phase != AppPhase::Launch {
            return;
        }

        let n = if state.flow.phase == AppPhase::Launch {
            if state.flow.launch_started_ms.is_none() {
                state.flow.launch_started_ms = Some(Globals::now());
            }
            let elapsed = state
                .flow
                .launch_started_ms
                .map(|start| Globals::now() - start)
                .unwrap_or(0.0);
            if elapsed >= 2000.0 {
                0
            } else {
                event.loop_count
            }
        } else {
            event.loop_count
        };

        #[rustfmt::skip]
        let art_cells: Vec<Vec<Block>> = vec![
            art_line(n + 2, " ____            _   _                 "),
            art_line(n + 3, "|  _ \\ _   _ ___| |_(_) ___ ___  _ __  "),
            art_line(n + 4, "| |_) | | | / __| __| |/ __/ _ \\| '_ \\ "),
            art_line(n + 5, "|  _ <| |_| \\__ \\ |_| | (_| (_) | | | |"),
            art_line(n + 2, "|_| \\_\\___,_|___/\\__|_|\\___\\___/|_| |_| "),
            vec![],
            bouncing_text(n),
        ];

        el.look(Look::from(art_cells));
        el.draw();
    });

    splash_logo
}
