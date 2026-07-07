use crate::SplashState;
use incredible::*;

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
    let color = (n % 5) as u8; // same color logic you used

    s.chars()
        .map(|c| {
            let d = Decor::default();
            d.color.replace(Some(color.into()));
            Block::new(c, d)
        })
        .collect()
}

pub fn build() -> Element<SplashState> {
    let splash_logo = Element::new();

    splash_logo.on_loop(|el, _state: &mut SplashState, event| {
        let n = event.loop_count as usize;

        let term_cols = Platform::columns();
        let term_rows = Platform::rows();
        let art_width = 39;
        let art_height = 7; // finishing with bounce text makes 7

        let x = ((term_cols.saturating_sub(art_width)) / 2) as isize;
        let y = ((term_rows.saturating_sub(art_height)) / 2) as isize;

        #[rustfmt::skip]
        let art_cells: Vec<Vec<Block>> = vec![
            art_line(n + 2, " ____            _   _                 "),
            art_line(n + 3, "|  _ \\ _   _ ___| |_(_) ___ ___  _ __  "),
            art_line(n + 4, "| |_) | | | / __| __| |/ __/ _ \\| '_ \\ "),
            art_line(n + 5, "|  _ <| |_| \\__ \\ |_| | (_| (_) | | | |"),
            art_line(n + 2, "|_| \\_\\___,_|___/\\__|_|\\___\\___/|_| |_| "),
            vec![],                                // empty spacer row
            bouncing_text(n as usize),
        ];

        el.look(Look::from(art_cells));
        el.x(x);
        el.y(y);

        el.draw();
    });

    splash_logo
}
