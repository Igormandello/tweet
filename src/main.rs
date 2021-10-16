use std::{error::Error, io};
use std::time::Duration;

use argh::FromArgs;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use termion::event::Key;
use tui::{backend::TermionBackend, Terminal};

use crate::app::App;
use crate::event::{Config, Event, Events};

mod app;
mod ui;
mod event;

#[derive(Debug, FromArgs)]
#[argh(description = "procrastinate directly from your terminal")]
struct CLI {
    #[argh(option, short = 'o', default = "false", description = "show just one tweet and nothing else")]
    only_one: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _cli: CLI = argh::from_env();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(200),
        ..Config::default()
    });

    let mut app = App::new("Tweet");
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    app.on_key(c);
                }
                _ => {}
            },
            Event::Tick => app.on_tick()
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
