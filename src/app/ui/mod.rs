pub(crate) mod centered_modal;
pub(crate) mod screen;
pub(crate) mod title_bar;

pub(crate) use crate::screens::editor::ui::button_16;
pub(crate) use crate::screens::editor::ui::button_8;
pub(crate) use crate::screens::editor::ui::button_exit;
pub(crate) use crate::screens::editor::ui::button_save;
pub(crate) use crate::screens::editor::ui::canvas_16;
pub(crate) use crate::screens::editor::ui::canvas_8;
pub(crate) use crate::screens::editor::ui::color_candidate;
pub(crate) use crate::screens::editor::ui::color_picker_16;
pub(crate) use crate::screens::editor::ui::color_picker_216;
pub(crate) use crate::screens::editor::ui::color_picker_empty;
pub(crate) use crate::screens::editor::ui::color_picker_gray;
pub(crate) use crate::screens::editor::ui::color_picker_palette;
pub(crate) use crate::screens::editor::ui::color_selected;
pub(crate) use crate::screens::editor::ui::label_color_candidate;
pub(crate) use crate::screens::editor::ui::label_color_selected;
pub(crate) use crate::screens::editor::ui::label_end;
pub(crate) use crate::screens::editor::ui::label_new;
pub(crate) use crate::screens::editor::ui::utils;

pub(crate) use crate::screens::splash::ui::splash_footer;
pub(crate) use crate::screens::splash::ui::splash_logo;

use crate::State;
use little_tui::*;

pub(crate) static APP_WIDTH: usize = 80;
pub(crate) static APP_HEIGHT: usize = 24;

pub(crate) fn reposition<S>(el: &Element<S>, x: isize, y: isize, state: &State) {
    if Terminal::columns() >= APP_WIDTH && Terminal::rows() >= APP_HEIGHT {
        el.x(state.app_x + x);
        el.y(state.app_y + y);
    }
}
