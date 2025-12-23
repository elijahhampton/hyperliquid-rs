use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;
use tokio::sync::mpsc;

pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    tx: mpsc::Sender<Event>,
}

impl EventHandler {
    pub fn new(tx: mpsc::Sender<Event>) -> Self {
        Self { tx }
    }

    pub async fn run(&self) {
        let mut tick_interval = tokio::time::interval(Duration::from_millis(250)); // CHANGED: 250ms instead of 2s

        loop {
            let event = if event::poll(Duration::from_millis(100)).unwrap() {
                match event::read().unwrap() {
                    CrosstermEvent::Key(key) => Event::Key(key),
                    _ => continue,
                }
            } else {
                tick_interval.tick().await;
                Event::Tick
            };

            if self.tx.send(event).await.is_err() {
                break;
            }
        }
    }
}
