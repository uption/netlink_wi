use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event;
use ratatui::prelude::Backend;
use ratatui::widgets::ListState;
use ratatui::Terminal;

use crate::handler::handle_key_events;
use crate::wifi_info::WifiInfoWorker;

/// Application state.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub wifi_info: WifiInfoWorker,
    pub list_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let wifi_info = WifiInfoWorker::start();

        Self {
            running: true,
            counter: 0,
            wifi_info,
            list_state: ListState::default(),
        }
    }

    /// Run the application main loop.
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.running {
            if self.list_state.selected().is_none() {
                self.list_state.select_first();
            }
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    /// Handle terminal events.
    fn handle_events(&mut self) -> Result<()> {
        if !event::poll(Duration::from_millis(100))? {
            return Ok(());
        }
        if let event::Event::Key(key_event) = event::read()? {
            handle_key_events(key_event, self)?;
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
