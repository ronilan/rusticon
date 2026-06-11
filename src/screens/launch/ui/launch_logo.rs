use crate::core::model::State;
use incredible::*;

fn text_line(s: &str) -> Vec<Block> {
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
    let launch_logo = Element::new();
    launch_logo.look(Look::from((40, 7)));

    #[rustfmt::skip]
    let art_cells: Vec<Vec<Block>> = vec![
        art_line(1, " ____            _   _                 "),
        art_line(2, "|  _ \\ _   _ ___| |_(_) ___ ___  _ __  "),
        art_line(3, "| |_) | | | / __| __| |/ __/ _ \\| '_ \\ "),
        art_line(4, "|  _ <| |_| \\__ \\ |_| | (_| (_) | | | |"),
        art_line(1, "|_| \\_\\___,_|___/\\__|_|\\___\\___/|_| |_| "),
        vec![],
        text_line("An icon editor for the terminal (and the web)"),
    ];

    launch_logo.look(Look::from(art_cells));

    launch_logo
}
