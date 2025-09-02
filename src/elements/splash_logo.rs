use crate::tui_engine::*;
use crate::SplashState;

fn bouncing_text(n: usize) -> String {
    let width = 40;
    let text = "An icon editor for the terminal";
    let text_len = text.len();

    if text_len >= width {
        return text.chars().take(width).collect();
    }

    let max_pad = width - text_len;
    let cycle = max_pad * 2;
    let pos = n % cycle;
    let pad = if pos <= max_pad { pos } else { cycle - pos };

    // Pad left and right to make the string exactly `width`
    let right_pad = width - text_len - pad;
    format!(
        "{:pad$}{}{:right_pad$}",
        "",
        text,
        "",
        pad = pad,
        right_pad = right_pad
    )
}

fn art_row(n: u8, s: &str) -> String {
    terminal_style::format::bold(
        terminal_style::format::color(n, s).unwrap_or_else(|_| s.to_string()),
    )
}

pub fn build<'a>() -> Element<'a, SplashState> {
    let mut splash_logo = Element::new(0, 0, Look::new());

    splash_logo.on_loop = Some(Box::new(|el, state: &mut SplashState, event| {
        let n = event.loop_count as u16;

        let term_cols = columns();
        let term_rows = rows();
        let art_width = 39;
        let art_height = 6;

        let x = (term_cols.saturating_sub(art_width)) / 2;
        let y = (term_rows.saturating_sub(art_height)) / 2;

        #[rustfmt::skip]
        let art = vec![
            vec![art_row(((n + 2) % 5) as u8, " ____            _   _                 ")],
            vec![art_row(((n + 3) % 5) as u8, "|  _ \\ _   _ ___| |_(_) ___ ___  _ __  ")],
            vec![art_row(((n + 4) % 5) as u8, "| |_) | | | / __| __| |/ __/ _ \\| '_ \\ ")],
            vec![art_row(((n + 5) % 5) as u8, "|  _ <| |_| \\__ \\ |_| | (_| (_) | | | |")],
            vec![art_row(((n + 2) % 5) as u8, "|_| \\_\\___,_|___/\\__|_|\\___\\___/|_| |_| ")],
            vec![String::new()],
            vec![bouncing_text(n as usize).to_string()],
        ];

        el.look.update(Look::from(art));
        el.x.set(x);
        el.y.set(y);

        draw(el);

        state.loop_count = event.loop_count
    }));

    splash_logo
}
