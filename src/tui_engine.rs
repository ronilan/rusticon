use std::{
    cell::{Cell, RefCell}, // Interior mutability
    io::{stdout, Write},   // For printing to terminal
    rc::Rc,                // Shared ownership of Elements
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{size, Clear, ClearType}, // Clear terminal
};

use terminal_style::format::stylable::Stylable;

use crate::event_loop::{start, Listener}; // Event loop runner & callbacks

// -----------------------------
// Look
// -----------------------------
// Represents a 2D grid of cells, each a String (can be multi-character)
// Use RefCell for interior mutability
#[derive(Clone, Default)]
pub struct Look {
    cells: RefCell<Vec<Vec<String>>>,
}

impl Look {
    // Creates an empty Look
    pub fn new() -> Self {
        Self {
            cells: RefCell::new(Vec::new()),
        }
    }

    // Borrow cells immutably
    pub fn cells(&self) -> std::cell::Ref<'_, Vec<Vec<String>>> {
        self.cells.borrow()
    }

    // Replace entire grid with new cells (generic)
    pub fn update<L: Into<Look>>(&self, new_look: L) {
        let new_look = new_look.into();
        self.cells.replace(new_look.cells.into_inner());
    }
}

// -----------------------------
// Conversions to Look
// -----------------------------
impl From<&str> for Look {
    fn from(s: &str) -> Self {
        let rows: Vec<Vec<String>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_string()).collect())
            .collect();
        Self {
            cells: RefCell::new(rows),
        }
    }
}

impl From<String> for Look {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&[String]> for Look {
    fn from(arr: &[String]) -> Self {
        Self {
            cells: RefCell::new(vec![arr.to_vec()]),
        }
    }
}

impl From<&[Vec<String>]> for Look {
    fn from(arr: &[Vec<String>]) -> Self {
        Self {
            cells: RefCell::new(arr.to_vec()),
        }
    }
}

impl From<&[&str]> for Look {
    fn from(arr: &[&str]) -> Self {
        Self {
            cells: RefCell::new(vec![arr.iter().map(|s| s.to_string()).collect()]),
        }
    }
}

impl From<&[Vec<&str>]> for Look {
    fn from(arr: &[Vec<&str>]) -> Self {
        Self {
            cells: RefCell::new(
                arr.iter()
                    .map(|row| row.iter().map(|s| s.to_string()).collect())
                    .collect(),
            ),
        }
    }
}

impl From<Vec<String>> for Look {
    fn from(row: Vec<String>) -> Self {
        Self {
            cells: RefCell::new(vec![row]),
        }
    }
}

impl From<Vec<Vec<String>>> for Look {
    fn from(cells: Vec<Vec<String>>) -> Self {
        Self {
            cells: RefCell::new(cells),
        }
    }
}

impl From<Vec<Vec<&str>>> for Look {
    fn from(cells: Vec<Vec<&str>>) -> Self {
        Self {
            cells: RefCell::new(
                cells
                    .into_iter()
                    .map(|row| row.into_iter().map(|s| s.to_string()).collect())
                    .collect(),
            ),
        }
    }
}

// Single-row construction from iterator
impl<T: ToString> FromIterator<T> for Look {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let row: Vec<String> = iter.into_iter().map(|item| item.to_string()).collect();
        Self {
            cells: RefCell::new(vec![row]),
        }
    }
}

impl<'a> Stylable for &'a Look {
    type Output = Look;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        (*self).apply(f)
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        (*self).apply_result(f)
    }
}

impl Stylable for Look {
    type Output = Look;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        let styled = self
            .cells()
            .iter()
            .map(|row| row.iter().map(|s| f(s)).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        Look {
            cells: RefCell::new(styled),
        }
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        let mut styled_rows = Vec::with_capacity(self.cells().len());

        for row in self.cells().iter() {
            let mut new_row = Vec::with_capacity(row.len());
            for s in row {
                new_row.push(f(s)?);
            }
            styled_rows.push(new_row);
        }

        Ok(Look {
            cells: RefCell::new(styled_rows),
        })
    }
}

// -----------------------------
// Elements container
// -----------------------------
// Holds multiple Element instances
#[derive(Clone)]
pub struct Elements<'a, S> {
    inner: Rc<RefCell<Vec<Element<'a, S>>>>, // Shared, mutable
}

impl<'a, S> Elements<'a, S> {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn push(&self, el: Element<'a, S>) {
        self.inner.borrow_mut().push(el);
    }

    pub fn iter(&self) -> std::cell::Ref<'_, Vec<Element<'a, S>>> {
        // `self.inner` is an `Rc<RefCell<Vec<Element<'a, S>>>>`.
        // `.borrow()` immutably borrows the inner Vec<Element>.
        // It returns a `Ref<'_, Vec<Element<'a, S>>>`, which implements `Deref<Target=Vec<...>>`.
        self.inner.borrow()
    }

    pub fn extend(&self, other: Elements<'a, S>) {
        let mut self_borrow = self.inner.borrow_mut();
        let mut other_borrow = other.inner.borrow_mut();
        self_borrow.append(&mut other_borrow);
    }
}

impl<'a, S> Default for Elements<'a, S> {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------
// Element
// -----------------------------
pub struct Element<'a, S> {
    pub x: Cell<u16>, // Position X
    pub y: Cell<u16>, // Position Y
    pub look: Look,   // Visual representation

    // Optional callbacks for events
    pub on_loop: Option<Box<dyn Fn(&Element<'a, S>, &mut S, &EventData) + 'a>>,
    pub on_keypress: Option<Box<dyn Fn(&Element<'a, S>, &mut S, &EventData) + 'a>>,
    pub on_move: Option<Box<dyn Fn(&Element<'a, S>, &mut S, &EventData) + 'a>>,
    pub on_click: Option<Box<dyn Fn(&Element<'a, S>, &mut S, &EventData) + 'a>>,
    pub on_state: Option<Box<dyn Fn(&Element<'a, S>, &S) + 'a>>,
}

impl<'a, S> Element<'a, S> {
    pub fn new(x: u16, y: u16, look: Look) -> Self {
        Self {
            x: Cell::new(x),
            y: Cell::new(y),
            look,
            on_loop: None,
            on_keypress: None,
            on_move: None,
            on_click: None,
            on_state: None,
        }
    }
}

impl<'a, S> Default for Element<'a, S> {
    fn default() -> Self {
        Self::new(0, 0, Look::new())
    }
}

// -----------------------------
// Helpers
// -----------------------------

// Check if mouse is over the element
// Assumes uniform row widths and single-character cells for precise detection
pub fn mouse_over<S>(el: &Element<S>, event: &EventData) -> bool {
    // Ensure valid mouse coordinates
    let (mx, my) = match (event.x, event.y) {
        (Some(x), Some(y)) => (x, y),
        _ => return false,
    };

    let cells = el.look.cells();
    if cells.is_empty() {
        return false;
    }

    let height = cells.len() as u16;
    let width = cells[0].len() as u16;
    if width == 0 {
        return false;
    }

    mx >= el.x.get() && mx < el.x.get() + width && my >= el.y.get() && my < el.y.get() + height
}

/// Returns the current terminal width in columns
pub fn columns() -> u16 {
    size()
        .map(|(cols, _rows)| cols)
        .expect("Failed to get terminal size")
}

/// Returns the current terminal height in rows
pub fn rows() -> u16 {
    size()
        .map(|(_cols, rows)| rows)
        .expect("Failed to get terminal size")
}

// Moves the terminal cursor to (x, y)
pub fn go_to(x: u16, y: u16) {
    let mut out = stdout();
    execute!(out, MoveTo(x, y)).expect("Failed to move cursor");
}

// Clears terminal below cursor
pub fn clear_below() {
    let mut out = stdout();
    execute!(out, Clear(ClearType::FromCursorDown)).expect("Failed to clear terminal below cursor");
}

// Clears entire terminal
pub fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).expect("Failed to clear terminal");
}

// -----------------------------
// Drawing
// -----------------------------

// Draws a single element - common use
pub fn draw<S>(el: &Element<S>) {
    let look_ref = el.look.cells();
    let mut out = stdout(); // Do not lock stdout. Needed for event capture.

    for (row_idx, row) in look_ref.iter().enumerate() {
        execute!(out, MoveTo(el.x.get(), el.y.get() + row_idx as u16)).unwrap();
        for cell in row {
            write!(out, "{}", cell).unwrap(); // Write each cell directly
        }
    }

    out.flush().unwrap();
}

// Draw all top-level elements (no nested elements)
pub fn draw_all<S>(elements: &Elements<S>) {
    let mut out = stdout(); // Do not lock stdout. Needed for event capture.

    for el in elements.inner.borrow().iter() {
        let look_ref = el.look.cells();

        for (row_idx, row) in look_ref.iter().enumerate() {
            execute!(out, MoveTo(el.x.get(), el.y.get() + row_idx as u16)).unwrap();
            for cell in row {
                write!(out, "{}", cell).unwrap();
            }
        }
    }

    out.flush().unwrap();
}

// -----------------------------
// Event Listener Builder
// -----------------------------
fn build_listeners<'a, S>(elements: &Elements<'a, S>) -> Vec<Listener<'a, S>>
where
    S: Clone + PartialEq + 'static,
{
    let mut listeners = Vec::new();
    let el_ref = elements.clone();

    let listener = Listener {
        on_loop: Box::new({
            let el_ref = el_ref.clone();
            move |state: &mut S, event| {
                let elements_borrow = el_ref.iter();
                for el in elements_borrow.iter() {
                    if let Some(cb) = &el.on_loop {
                        cb(el, state, &event);
                    }
                }
            }
        }),
        on_keypress: Box::new({
            let el_ref = el_ref.clone();
            move |state: &mut S, event| {
                let elements_borrow = el_ref.iter();
                for el in elements_borrow.iter() {
                    if let Some(cb) = &el.on_keypress {
                        cb(el, state, &event);
                    }
                }
            }
        }),
        on_move: Box::new({
            let el_ref = el_ref.clone();
            move |state: &mut S, event| {
                let elements_borrow = el_ref.iter();
                for el in elements_borrow.iter() {
                    if let Some(cb) = &el.on_move {
                        cb(el, state, &event);
                    }
                }
            }
        }),
        on_click: Box::new({
            let el_ref = el_ref.clone();
            move |state: &mut S, event| {
                let elements_borrow = el_ref.iter();
                for el in elements_borrow.iter() {
                    if let Some(cb) = &el.on_click {
                        cb(el, state, &event);
                    }
                }
            }
        }),
        on_state: Box::new({
            let el_ref = el_ref.clone();
            move |state: &S| {
                let elements_borrow = el_ref.iter(); // borrow the Vec<Element>
                for el in elements_borrow.iter() {
                    if let Some(cb) = &el.on_state {
                        cb(el, state);
                    }
                }
            }
        }),
    };

    listeners.push(listener);
    listeners
}

// -----------------------------
// TUI Runner
// -----------------------------
pub fn run<'a, S>(
    state: S,
    elements: Elements<'a, S>,
    tick_rate: Option<Duration>,
    alt_exit: Option<&dyn Fn(&S) -> bool>,
) -> S
// <-- return the final state
where
    S: Clone + PartialEq + 'static,
{
    clear_screen();
    // initial draw as defined by user
    draw_all(&elements);

    let mut listeners = build_listeners(&elements);

    let tick_rate = tick_rate.unwrap_or_else(|| Duration::from_millis(33));

    // simply return the result of start
    start(state, &mut listeners, tick_rate, alt_exit)
}

// Re-export EventData for convenience
pub use crate::event_loop::EventData;
