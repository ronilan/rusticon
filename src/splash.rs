use terminal_style::format::{bold, color, faint};

use crate::tui_engine::{columns, draw, rows, Element, Elements, Look};

use crate::SplashState;

fn sliding_text(n: usize) -> String {
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

pub fn build_elements<'a>() -> Elements<'a, SplashState> {
    let elements: Elements<SplashState> = Elements::new();

    // Splash
    let mut splash = Element::new(0, 0, Look::new());

    splash.on_loop = Some(Box::new(|el, state: &mut SplashState, event| {
        let n = event.loop_count as u16;

        let term_cols = columns();
        let term_rows = rows();
        let art_width = 39;
        let art_height = 6;

        let x = (term_cols.saturating_sub(art_width)) / 2;
        let y = (term_rows.saturating_sub(art_height)) / 2;

        #[rustfmt::skip]
        let art = vec![
            vec![bold(&color(((n + 2) % 5) as u8, " ____            _   _                 ").unwrap())],
            vec![bold(&color(((n + 3) % 5) as u8, "|  _ \\ _   _ ___| |_(_) ___ ___  _ __  ").unwrap())],
            vec![bold(&color(((n + 4) % 5) as u8, "| |_) | | | / __| __| |/ __/ _ \\| '_ \\ ").unwrap())],
            vec![bold(&color(((n + 5) % 5) as u8, "|  _ <| |_| \\__ \\ |_| | (_| (_) | | | |").unwrap())],
            vec![bold(&color(((n + 2) % 5) as u8, "|_| \\_\\___,_|___/\\__|_|\\___\\___/|_| |_| ").unwrap())],
            vec![format!("{:>39}", "").to_string()],
            vec![sliding_text(n as usize).to_string()],
        ];

        el.look.update(Look::from(art));
        el.x.set(x);
        el.y.set(y);

        draw(el);

        state.loop_count = event.loop_count
    }));

    elements.push(splash);

    // Footer
    let mut footer = Element::new(0, 0, Look::new());

    footer.on_loop = Some(Box::new(|el, _state, _event| {
        let term_cols = columns();
        let term_rows = rows();

        let text = "                     Made with Rust                  ";
        el.x.set((term_cols.saturating_sub(text.len() as u16)) / 2);
        el.y.set(term_rows.saturating_sub(1));
        el.look.update(bold(faint(Look::from(text))));

        draw(el)
    }));

    elements.push(footer);

    elements
}
