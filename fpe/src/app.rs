// Application
#[derive(Debug, Default)]
pub struct App {
    counter: f64,
    should_quit: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
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
}
