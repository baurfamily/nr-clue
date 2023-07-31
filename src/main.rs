mod nerdgraph;

use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};


#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Query,
    Chart,
    Settings,
}

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



// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = nerdgraph::Client::new();

//     let actor = client.actor_info().await;

//     if let Some(actor) = actor {
//         println!("Num of Accounts: {}", actor.accounts.len());
//         println!("First Account: {} ({})", actor.accounts[0].name, actor.accounts[0].id);
//         println!("User name: {}", actor.user.name);
//     }

//     let results = client.nrql(265881,"SElECT count(*) FROM Transaction FACET name".to_string()).await;

//     if let Some(results) = results {
//         // println!("results: {:?}", results);
//         for hash in &results {
//             println!("------------------");
//             for (key, value) in hash {
//                 match value {
//                     nerdgraph::JsonValue::String(value) => println!("\t{} = {}", key,  value) ,
//                     nerdgraph::JsonValue::Integer(value) => println!("\t{} = {}", key, value) ,
//                     nerdgraph::JsonValue::Boolean(value) => println!("\t{} = {}", key,  value) ,
//                     nerdgraph::JsonValue::Float(value) => println!("\t{} = {}", key,  value) ,
//                 }

//             }
//         }
//     }

//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = build_layout(size);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Query => rect.render_widget(build_query(), chunks[1]),
                MenuItem::Chart => rect.render_widget(build_chart(), chunks[1]),
                MenuItem::Settings => rect.render_widget(build_settings(), chunks[1]),
            }
            rect.render_widget(build_menu(active_menu_item), chunks[0]);
            rect.render_widget( build_status(), chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('Q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('H') => active_menu_item = MenuItem::Home,
                KeyCode::Char('q') => active_menu_item = MenuItem::Query,
                KeyCode::Char('c') => active_menu_item = MenuItem::Chart,
                KeyCode::Char('s') => active_menu_item = MenuItem::Settings,
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn build_layout(size: Rect) -> Vec<Rect> {
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
}

fn build_menu<'a>(active_menu_item: MenuItem) -> Tabs<'a> {
    let menu_titles = vec!["Home", "query", "chart", "settings", "Quit"];


    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
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


fn build_status<'a>() -> Paragraph<'a> {
   Paragraph::new("current status goes here")
   .block(
    Block::default()
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::White))
    .title("Home")
    .border_type(BorderType::Plain),
   )
}

fn build_query<'a>() -> Paragraph<'a> {
    Paragraph::new("type query here")
    .block(
     Block::default()
     .borders(Borders::ALL)
     .style(Style::default().fg(Color::White))
     .title("Home")
     .border_type(BorderType::Plain),
    )
 }



fn build_chart<'a>() -> Paragraph<'a> {
    Paragraph::new("data chart goes here")
    .block(
     Block::default()
     .borders(Borders::ALL)
     .style(Style::default().fg(Color::White))
     .title("Home")
     .border_type(BorderType::Plain),
    )
 }


 fn build_settings<'a>() -> Paragraph<'a> {
    Paragraph::new("settings go here")
    .block(
     Block::default()
     .borders(Borders::ALL)
     .style(Style::default().fg(Color::White))
     .title("Home")
     .border_type(BorderType::Plain),
    )
 }



fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
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
