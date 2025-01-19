use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        KeyCode::Up => {
            app.list_state.select_previous();
        }
        KeyCode::Down => {
            app.list_state.select_next();
        }
        _ => {}
    }
    Ok(())
}
