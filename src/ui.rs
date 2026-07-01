use ratatui::{
    layout::{Alignment, Constraint, Layout}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph, Widget}
};

use crate::app::{App, ApplicationMode::{*}};
use crate::finfo::FloatComponents;

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized
    {
        match self.mode() {
            Counter => self.render_counter_mode(area, buf),
            EnterNumber => self.render_enter_number_mode(area, buf),
            SelectSpecial => {},
            ParseError => self.render_parse_error(area, buf),
        }
    }
}

impl App {
    fn render_enter_number_mode(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50), 
                Constraint::Percentage(40),
                Constraint::Percentage(10),
            ])
            .split(area);

        // Render counter
        FloatComponents::new(self.counter(), self.display_base()).render(layout[0], buf);
        // render instructions
        render_instructions(layout[1], buf);
        // Render input box
        Paragraph::new(
            self.input_prompt()
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left)
        .render(layout[2], buf);
    }

    fn render_parse_error(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50), 
                Constraint::Percentage(40),
                Constraint::Percentage(10),
            ])
            .split(area);

        // Render counter
        FloatComponents::new(self.counter(), self.display_base()).render(layout[0], buf);
        // render instructions
        render_instructions(layout[1], buf);
        // Render input box
        Paragraph::new(
            "Invalid number format, press any key to continue"
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .render(layout[2], buf);

    }

    fn render_counter_mode(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Render counter
        FloatComponents::new(self.counter(), self.display_base()).render(layout[0], buf);
        // render instructions
        render_instructions(layout[1], buf);

    }
}


fn render_instructions(area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Press `x` for hex display and `b` for binary display.\n\
        Press `c` to enter a specific number for anaylysis\n\
      ",
        )
        .block(
            Block::default()
                .title(" Instructions ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .render(area, buf);
}
