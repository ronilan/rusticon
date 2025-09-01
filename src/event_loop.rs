use std::{
    cell::RefCell,                           // Interior mutability for shared state
    io::stdout,                              // Terminal output
    panic::{catch_unwind, AssertUnwindSafe}, // Safe panic handling
    rc::Rc,                                  // Shared ownership of state
    time::Duration,
};

use crossterm::{
    cursor,
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent,
        KeyModifiers, MouseButton, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, ClearType},
};

use futures::{executor::block_on, future::FutureExt, StreamExt};
use futures_timer::Delay;

// -----------------------------
// Key Mapping
// -----------------------------

/// Converts KeyModifiers into human-friendly string representation
fn modifiers_map(modifiers: KeyModifiers) -> Vec<String> {
    let mut result = Vec::new();
    if modifiers.contains(KeyModifiers::CONTROL) {
        result.push("ctrl".to_string());
    }
    if modifiers.contains(KeyModifiers::SHIFT) {
        result.push("shift".to_string());
    }
    if modifiers.contains(KeyModifiers::ALT) {
        result.push("alt".to_string());
    }
    if modifiers.contains(KeyModifiers::META) {
        result.push("meta".to_string());
    }
    result
}

/// Converts raw crossterm KeyEvent into human-friendly string
fn key_map(key: &KeyEvent) -> Option<String> {
    use KeyCode::*;
    match key.code {
        Enter => Some("enter".into()),
        Up => Some("up".into()),
        Down => Some("down".into()),
        Left => Some("left".into()),
        Right => Some("right".into()),
        Esc => Some("escape".into()),
        Backspace => Some("delete".into()),
        F(n @ 1..=12) => Some(format!("f{}", n)), // function keys
        Char(c) => Some(c.to_string()),           // any typed char
        _ => None,
    }
}

// -----------------------------
// Event Data
// -----------------------------

/// Structure passed to callbacks on each loop iteration or event
#[derive(Clone, Debug, Default)]
pub struct EventData {
    pub loop_count: usize,      // Number of loop iterations
    pub key: Option<String>,    // Last pressed key
    pub modifiers: Vec<String>, // Pressed modifiers, e.g., ["ctrl", "shift"]
    pub x: Option<u16>,         // Mouse X coordinate if relevant
    pub y: Option<u16>,         // Mouse Y coordinate if relevant
}

// -----------------------------
// Listener Struct
// -----------------------------

/// A Listener contains callback hooks for different event types
pub struct Listener<'a, S: 'a> {
    pub on_loop: Box<dyn FnMut(&mut S, EventData) + 'a>,
    pub on_keypress: Box<dyn FnMut(&mut S, EventData) + 'a>,
    pub on_move: Box<dyn FnMut(&mut S, EventData) + 'a>,
    pub on_click: Box<dyn FnMut(&mut S, EventData) + 'a>,
    pub on_state: Box<dyn FnMut(&S) + 'a>,
}

impl<'a, S: 'a> Listener<'a, S> {
    pub fn new() -> Self {
        Self {
            on_loop: Box::new(|_s, _| {}),
            on_keypress: Box::new(|_s, _| {}),
            on_move: Box::new(|_s, _| {}),
            on_click: Box::new(|_s, _| {}),
            on_state: Box::new(|_| {}),
        }
    }
}

impl<'a, S: 'a> Default for Listener<'a, S> {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------
// Terminal Setup / Teardown
// -----------------------------

/// Configure terminal for raw mode + mouse tracking
pub fn setup() {
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = stdout();

    execute!(
        stdout,
        cursor::Hide,         // hide cursor
        cursor::MoveTo(0, 0), // reset cursor
        EnableMouseCapture    // enable mouse events
    )
    .expect("Failed to initialize terminal");
}

/// Restore terminal to normal state
fn teardown() {
    let mut stdout = stdout();

    execute!(
        stdout,
        crossterm::style::ResetColor,               // reset text color
        cursor::Show,                               // show cursor
        crossterm::terminal::Clear(ClearType::All), // clear screen
        cursor::MoveTo(0, 0),                       // move cursor to top-left
        DisableMouseCapture                         // disable mouse events
    )
    .expect("Failed to reset terminal");

    disable_raw_mode().ok();
}

// -----------------------------
// Main Event Loop
// -----------------------------

/// Runs the event loop until exit
/// - `state`: initial application state
/// - `listeners`: collection of callback hooks
///  - `tick_rate`: how often to tick when no events occur
pub fn start<'a, S>(
    state: S,
    listeners: &mut [Listener<'a, S>],
    tick_rate: Duration,
    alt_exit: Option<&dyn Fn(&S) -> bool>,
) -> S
where
    S: Clone + PartialEq + 'static,
{
    // ---------------- SAFE SETUP ----------------
    // Wrap setup in catch_unwind to restore terminal if panic occurs
    let setup_result = catch_unwind(AssertUnwindSafe(setup));
    if let Err(panic_info) = setup_result {
        teardown();
        std::panic::resume_unwind(panic_info);
    }

    // Use Rc<RefCell> to allow interior mutability across loop and events
    let state = Rc::new(RefCell::new(state));
    let mut events = EventStream::new();

    // Notify listeners of initial state
    // Will allow downstream users of event loop to draw initial UI
    {
        let s = state.borrow();
        for l in listeners.iter_mut() {
            (l.on_state)(&s);
        }
    }

    let mut loop_count: usize = 0;
    let mut should_exit = false;

    // ---------------- MAIN LOOP ----------------
    let result = catch_unwind(AssertUnwindSafe(|| {
        while !should_exit {
            // --- LOOP CALLBACK ---
            {
                let old_state = state.borrow().clone();
                for l in listeners.iter_mut() {
                    let mut s = state.borrow_mut();
                    (l.on_loop)(
                        &mut s,
                        EventData {
                            loop_count,
                            ..Default::default()
                        },
                    );
                }

                // Check optional exit
                if let Some(exit_fn) = alt_exit {
                    if exit_fn(&state.borrow()) {
                        should_exit = true;
                    }
                }

                // Notify listeners if state changed
                let new_state = state.borrow();
                if *new_state != old_state {
                    for l in listeners.iter_mut() {
                        (l.on_state)(&new_state);
                    }
                }
            }

            loop_count += 1;

            // --- EVENT HANDLING ---
            let maybe_event: Option<Result<Event, std::io::Error>> = block_on(async {
                futures::select! {
                    event = events.next().fuse() => event,       // terminal event
                    _ = Delay::new(tick_rate).fuse() => None,   // timeout → just return None
                }
            });

            // Handle EOF gracefully
            if let Some(event_result) = maybe_event {
                match event_result {
                    Ok(event) => match event {
                        // ----- KEY EVENTS -----
                        Event::Key(key_event) => {
                            // Ctrl-C exits loop only if no alt_exit was provided
                            if alt_exit.is_none()
                                && key_event.code == KeyCode::Char('c')
                                && key_event.modifiers.contains(KeyModifiers::CONTROL)
                            {
                                should_exit = true;
                                continue;
                            }

                            let key = key_map(&key_event);
                            let old_state = state.borrow().clone();

                            for l in listeners.iter_mut() {
                                let mut s = state.borrow_mut();
                                if let Some(ref k) = key {
                                    (l.on_keypress)(
                                        &mut s,
                                        EventData {
                                            loop_count,
                                            key: Some(k.clone()),
                                            modifiers: modifiers_map(key_event.modifiers),
                                            ..Default::default()
                                        },
                                    );
                                }
                            }

                            // Notify if state changed
                            let new_state = state.borrow();
                            if *new_state != old_state {
                                for l in listeners.iter_mut() {
                                    (l.on_state)(&new_state);
                                }
                            }
                            continue;
                        }

                        // ----- MOUSE EVENTS -----
                        Event::Mouse(mouse_event) => {
                            let pos = (mouse_event.column, mouse_event.row);
                            let old_state = state.borrow().clone();

                            for l in listeners.iter_mut() {
                                let mut s = state.borrow_mut();
                                // Trigger on_move ONLY when the mouse actually moved
                                if matches!(mouse_event.kind, MouseEventKind::Moved) {
                                    (l.on_move)(
                                        &mut s,
                                        EventData {
                                            loop_count,
                                            x: Some(pos.0),
                                            y: Some(pos.1),
                                            modifiers: modifiers_map(mouse_event.modifiers),
                                            ..Default::default()
                                        },
                                    );
                                }

                                // Trigger on_click for left-up or drag
                                if matches!(
                                    mouse_event.kind,
                                    MouseEventKind::Up(MouseButton::Left)
                                        | MouseEventKind::Drag(MouseButton::Left)
                                ) {
                                    (l.on_click)(
                                        &mut s,
                                        EventData {
                                            loop_count,
                                            x: Some(pos.0),
                                            y: Some(pos.1),
                                            modifiers: modifiers_map(mouse_event.modifiers),
                                            ..Default::default()
                                        },
                                    );
                                }
                            }

                            // Notify if state changed
                            let new_state = state.borrow();
                            if *new_state != old_state {
                                for l in listeners.iter_mut() {
                                    (l.on_state)(&new_state);
                                }
                            }
                            continue;
                        }

                        _ => {} // Ignore other events
                    },
                    Err(_) => {
                        // Treat IO errors from EventStream as EOF → exit gracefully
                        should_exit = true;
                    }
                }
            }
            // If maybe_event is None, that means it was a tick → do nothing, loop continues
        }
    }));

    // Ensure terminal restored even if panic occurs
    let _ = catch_unwind(AssertUnwindSafe(teardown));

    // Re-raise any panic from inside loop
    if let Err(panic_info) = result {
        std::panic::resume_unwind(panic_info);
    }

    // Return final state exactly as in original code
    let final_state = state.borrow().clone();
    final_state
}
