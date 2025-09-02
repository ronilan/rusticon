pub(crate) mod utils;

// --- Single and Vectors of Elements
pub(crate) mod splash_footer;
pub(crate) mod splash_logo;

pub(crate) mod centered_modal;
pub(crate) mod screen;
pub(crate) mod title_bar;

pub(crate) mod canvas_16;
pub(crate) mod canvas_8;

pub(crate) mod color_picker_16;
pub(crate) mod color_picker_216;
pub(crate) mod color_picker_empty;
pub(crate) mod color_picker_gray;
pub(crate) mod color_picker_palette;

pub(crate) mod label_color_selected;
pub(crate) mod color_selected;
pub(crate) mod color_candidate;
pub(crate) mod label_color_candidate;

pub(crate) mod button_16;
pub(crate) mod button_8;
pub(crate) mod label_new;

pub(crate) mod button_exit;
pub(crate) mod button_save;
pub(crate) mod label_end;

// --- Utility
use crate::{
    tui_engine::{columns, draw, rows, Element},
    AppState,
};

pub(crate) static APP_WIDTH: u16 = 80; // Width of the game window
pub(crate) static APP_HEIGHT: u16 = 24; // Height of the game window

pub(crate) fn draw_relative<S>(el: &Element<S>, x: u16, y: u16, state: &AppState) {
    if columns() >= APP_WIDTH && rows() >= APP_HEIGHT {
        el.x.set(state.app_x + x);
        el.y.set(state.app_y + y);
        draw(el);
    }
}
