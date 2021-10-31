#[macro_use]
extern crate lazy_static;

use std::{error::Error, io};

use argh::FromArgs;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use termion::event::Key;
use tui::{backend::TermionBackend, Terminal};

use crate::app::App;
use crate::event::{Event, Events};

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

    let events = Events::new();

    let mut app = App::new();
    terminal.draw(|f| ui::draw(f, &mut app))?;

    loop {
        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => app.on_key(c),
                Key::Right => app.next_tweet(),
                Key::Left => app.previous_tweet(),
                _ => continue
            },
            Event::Tick => app.on_tick()
        }

        terminal.draw(|f| ui::draw(f, &mut app))?;

        if !app.running {
            break;
        }
    }

    Ok(())
}
