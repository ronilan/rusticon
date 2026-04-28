use little_tui::*;
use little_tui_elements::Text;

use crate::core::model::{AppPhase, State};

fn render_title(el: &Text<State>, state: &State) {
    let cols = Platform::columns();
    let mut line = " ".repeat(cols);
    let text = format!(
        "Rusticon: {} {}x{}",
        state.editor.file_path, state.editor.size, state.editor.size
    );
    line.replace_range(0..text.len().min(cols), &text);
    el.text(&line);
}

pub fn build() -> Text<State> {
    let title_bar: Text<State> = Text::default();
    title_bar.x(0).y(0).showed(false);
    title_bar.inverse(true).fused(true);

    title_bar.on_state(|el, state| {
        let visible = !state.flow.viewport_too_small && state.flow.phase == AppPhase::Main;
        el.showed(visible);
        if visible {
            render_title(el, state);
        }
    });
    title_bar.on_window(|el, state, event| {
        if event.window == Window::Resize {
            render_title(el, state);
        }
    });

    title_bar
}
