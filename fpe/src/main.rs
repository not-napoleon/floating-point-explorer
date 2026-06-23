use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
    /*
    let val: f64 = 10.87;
    let fbits: u64 = val.to_bits();
    println!("binary:   {0:#064b}", fbits);
    let r: FloatComponents = deconstruct(val);
    println!("sign:     {0:#b} ({0})", r.sign);
    println!("exponent: {0:#011b} ({0})", r.exponent);
    println!("offset:   {0:#0b} ({0})", r.offset);
    */
}

#[derive(Debug, Default)]
pub struct App {
    counter: f64,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }
    
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter = self.counter.next_up();
    }

    fn decrement_counter(&mut self) {
        self.counter = self.counter.next_down();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Floating Point Explorer ".bold());
        let instructions = Line::from(vec![
            " Next Down ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // TODO: replace this with showing the details of the number
        let counter_text = Text::from(vec![Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

struct FloatComponents {
    sign: u64,
    exponent: u64,
    offset: u64
}

fn deconstruct(n: f64) -> FloatComponents {
    let sign_mask: u64 =     0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let exponent_mask: u64 = 0b0111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let offset_mask: u64 =   0b1000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    let bits: u64 = n.to_bits();

    let result: FloatComponents = FloatComponents {
        sign: (bits & sign_mask) >> 63,
        exponent: (bits & exponent_mask) >> 52,
        offset: bits & offset_mask
    };
    return result;
}

