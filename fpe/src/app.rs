use crate::finfo::DisplayBase::{self, Hex};

// Application
#[derive(Debug)]
pub struct App {
    counter: f64,
    should_quit: bool,
    display_base: DisplayBase,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            counter: 0.0,
            should_quit: false,
            display_base: Hex,
        }

    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn counter(&self) -> f64 {
        self.counter
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.next_up();
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.next_down();
    }

    pub fn display_base(&self) -> DisplayBase {
        self.display_base
    }

    pub fn binary_display(&mut self) {
        self.display_base = DisplayBase::Binary
    }

    pub fn hex_display(&mut self) {
        self.display_base = DisplayBase::Hex
    }
}

impl Default for App {
    fn default() -> App {
        App::new()
    }
}
