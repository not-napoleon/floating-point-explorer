use ratatui::{
    layout::{Alignment, Constraint, Layout}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph, Widget}
};

use crate::app::App;
use crate::finfo::FloatComponents;

impl Widget for &mut App {
   fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized
    {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        // render instructions
        Paragraph::new(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Press `x` for hex display and `b` for binary display.\n\
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
        .render(layout[1], buf);

        // Render counter
        FloatComponents::new(self.counter(), self.display_base()).render(layout[0], buf);
    } 
}

