use chrono::{DateTime, Local};
use std::collections::VecDeque;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    log: VecDeque<DateTime<Local>>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // Setup the timer event subscription
        subscribe(&[EventType::Timer, EventType::KeyPress]);

        // Set how many seconds out from here you want to trigger the timer
        set_timeout(1.0);
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::Timer(sec) => {
                // A timeout was triggered. Save current time to state
                self.log.push_front(Local::now());

                // Set timeout again
                set_timeout(1.0);
            }
            Event::KeyPress(key) => {
                // Clear log if ctrl+l is pressed
                if key == Key::Ctrl('l') {
                    self.log.clear();
                }
            }
            _ => panic!("Error"),
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        for time in &self.log {
            println!("Time: {}", time.format("%T"));
        }
    }
}
