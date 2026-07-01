use crate::{app::ApplicationMode::{Counter, EnterNumber, ParseError}, finfo::DisplayBase::{self, Hex}};

#[derive(Debug, Clone, Copy)]
pub enum ApplicationMode {
    Counter,
    EnterNumber,
    SelectSpecial,
    ParseError,
}

// Application
#[derive(Debug)]
pub struct App {
    counter: f64,
    should_quit: bool,
    display_base: DisplayBase,
    mode: ApplicationMode,
    input_text: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            counter: 0.0,
            should_quit: false,
            display_base: Hex,
            mode: ApplicationMode::Counter,
            input_text: String::new(),
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

    pub fn set_counter(&mut self, num: f64) {
        self.counter = num
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

    pub fn mode(&self) -> ApplicationMode {
        self.mode
    }

    fn set_mode(&mut self, mode: ApplicationMode) {
        self.mode = mode
    }

    pub fn start_enter_mode(&mut self) {
        self.reset_input();
        self.set_mode(EnterNumber);
    }

    pub fn end_enter_mode(&mut self) {
        match self.input_text.parse() {
            Ok(v) => {
                self.set_counter(v);
                self.set_mode(Counter);
            },
            Err(_) => self.set_mode(ParseError)
        }
    }

    pub fn reset_input(&mut self){
        self.input_text = String::new()
    }

    pub fn append_input_char(&mut self, c: char) {
        self.input_text.push(c)
    }

    pub fn input_prompt(&self) -> String {
            format!("Enter a number: {0}", self.input_text)
    }
}

impl Default for App {
    fn default() -> App {
        App::new()
    }
}
