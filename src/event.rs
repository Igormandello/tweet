use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);

        let _input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };

        let _tick_handle = {
            thread::spawn(move || loop {
                if let Err(err) = tx.send(Event::Tick) {
                    eprintln!("{}", err);
                    break;
                }

                thread::sleep(tick_rate);
            })
        };

        Events { rx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}