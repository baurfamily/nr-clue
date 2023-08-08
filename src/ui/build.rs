use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{ Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};

use crate::ui::MenuItem;

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Query => 1,
            MenuItem::Chart => 2,
            MenuItem::Settings => 3,
        }
    }
}

pub fn home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from("Welcome"),
        Line::from("to"),
        Line::from(""),
        Line::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        ),
        Line::from(""),
        Line::from("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet."),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

pub fn layout(size: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size)
        .to_vec()
}

pub fn menu<'a>(active_menu_item: MenuItem) -> Tabs<'a> {
    let menu_titles = vec!["Home", "query", "chart", "settings", "Quit"];

    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}

pub fn status<'a>() -> Paragraph<'a> {
    Paragraph::new("current status goes here").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    )
}

pub fn query<'a>() -> Paragraph<'a> {
    Paragraph::new("type query here").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    )
}

// pub fn query<'a>() -> dyn Widget<'a> {
//     let mut textarea = TextArea::default();
//     textarea.set_block(
//         Block::default()
//             .borders(Borders::ALL)
//             .title("Crossterm Minimal Example"),
//     );

//     textarea.widget()
// }

pub fn chart<'a>() -> Paragraph<'a> {
    Paragraph::new("data chart goes here").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    )
}

pub fn settings<'a>() -> Paragraph<'a> {
    Paragraph::new("settings go here").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    )
}
