use crate::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(1), Constraint::Length(3)],
    )
    .split(frame.size());

    let messages_block = Block::default().borders(Borders::ALL).title("Chat");
    let messages_list: Vec<ListItem> = app
        .messages
        .iter()
        .map(|message| {
            ListItem::new(Line::from(vec![
                Span::styled(&message.author_name, Style::default().fg(Color::Green)),
                ": ".into(),
                Span::raw(&message.content),
            ]))
        })
        .collect();

    let input_block = Block::default().borders(Borders::ALL).title("Input");
    let input_text = Paragraph::new(app.input_value.as_str()).block(input_block);
    frame.render_widget(input_text, layout[1]);

    let list = List::new(messages_list).block(messages_block);
    frame.render_widget(list, layout[0])
}
