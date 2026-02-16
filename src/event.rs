use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use tokio::sync::{mpsc, watch};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    tick_rate_tx: watch::Sender<u64>,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let (tick_rate_tx, mut tick_rate_rx) = watch::channel(tick_rate_ms);

        tokio::spawn(async move {
            let mut tick_interval =
                tokio::time::interval(Duration::from_millis(tick_rate_ms));

            loop {
                tokio::select! {
                    _ = tick_interval.tick() => {
                        if tx.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    Ok(()) = tick_rate_rx.changed() => {
                        let new_rate = *tick_rate_rx.borrow();
                        tick_interval = tokio::time::interval(Duration::from_millis(new_rate));
                        tick_interval.reset();
                    }
                    _ = tokio::task::spawn_blocking(|| {
                        event::poll(Duration::from_millis(50)).unwrap_or(false)
                    }) => {
                        if event::poll(Duration::ZERO).unwrap_or(false) {
                            if let Ok(evt) = event::read() {
                                let mapped = match evt {
                                    CrosstermEvent::Key(key) => Some(Event::Key(key)),
                                    CrosstermEvent::Mouse(mouse) => Some(Event::Mouse(mouse)),
                                    CrosstermEvent::Resize(w, h) => Some(Event::Resize(w, h)),
                                    _ => None,
                                };
                                if let Some(e) = mapped {
                                    if tx.send(e).is_err() {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Self { rx, tick_rate_tx }
    }

    pub fn set_tick_rate(&self, ms: u64) {
        let _ = self.tick_rate_tx.send(ms);
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("Event channel closed"))
    }
}
