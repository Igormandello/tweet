#[macro_use]
extern crate lazy_static;

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
mod twitter;

#[derive(Debug, FromArgs)]
#[argh(description = "procrastinate directly from your terminal")]
struct CLI {
    #[argh(option, short = 'o', default = "false", description = "show just one tweet and nothing else")]
    only_one: bool,
}


fn main() -> Result<(), Box<dyn Error>> {
    let _cli: CLI = argh::from_env();

    let terminal_builder = |x| Terminal::new(TermionBackend::new(AlternateScreen::from(MouseTerminal::from(x))));
    let mut terminal = terminal_builder(io::stdout().into_raw_mode()?)?;

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(200),
        ..Config::default()
    });

    let mut app = App::new();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => app.on_key(c),
                Key::Right => app.next_tweet(),
                Key::Left => app.previous_tweet(),
                _ => {}
            },
            Event::Tick => app.on_tick()
        }

        if !app.running {
            break;
        }
    }

    Ok(())
}
