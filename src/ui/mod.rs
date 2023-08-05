pub mod build;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Query,
    Chart,
    Settings,
}

pub fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
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
            let chunks = build::layout(size);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(build::home(), chunks[1]),
                MenuItem::Query => rect.render_widget(build::query(), chunks[1]),
                MenuItem::Chart => rect.render_widget(build::chart(), chunks[1]),
                MenuItem::Settings => rect.render_widget(build::settings(), chunks[1]),
            }
            rect.render_widget(build::menu(active_menu_item), chunks[0]);
            rect.render_widget(build::status(), chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('H') => active_menu_item = MenuItem::Home,
                KeyCode::Char('q') => active_menu_item = MenuItem::Query,
                KeyCode::Char('c') => active_menu_item = MenuItem::Chart,
                KeyCode::Char('s') => active_menu_item = MenuItem::Settings,
                KeyCode::Char('Q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
