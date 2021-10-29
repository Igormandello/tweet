pub struct App {
    pub running: bool,
    pub current_tweet: Option<String>,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            current_tweet: Option::None,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.running = false,
            'p' => self.current_tweet = Option::Some(String::from("The library is based on the principle of immediate rendering with intermediate buffers. This means that at each new frame you should build all widgets that are supposed to be part of the UI. While providing a great flexibility for rich and interactive UI, this may introduce over.")),
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}