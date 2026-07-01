use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, ApplicationMode::{*}};

pub fn update(app: &mut App, key_event: KeyEvent) {
        match app.mode() {
            Counter => update_counter_mode(app, key_event),
            EnterNumber => update_enter_number_mode(app, key_event),
            SelectSpecial => {},
            ParseError => {},
        }
}

pub fn update_enter_number_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter => app.end_enter_mode(),
        KeyCode::Char('0') | 
        KeyCode::Char('1') | 
        KeyCode::Char('2') | 
        KeyCode::Char('3') | 
        KeyCode::Char('4') | 
        KeyCode::Char('5') | 
        KeyCode::Char('6') | 
        KeyCode::Char('7') | 
        KeyCode::Char('8') | 
        KeyCode::Char('9') | 
        KeyCode::Char('.') => app.append_input_char(key_event.code.as_char().unwrap()), 
        _ => {}
    }
}

pub fn update_counter_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
        KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
        KeyCode::Char('x') => app.hex_display(),
        KeyCode::Char('b') => app.binary_display(),
        KeyCode::Char('c') => app.start_enter_mode(),
        _ => {}
    };
}
