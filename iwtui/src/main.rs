use color_eyre::Result;

use crate::app::App;

pub mod app;
pub mod handler;
pub mod ui;
pub mod wifi_info;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new();

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//     use handler::handle_key_events;
//     use insta::assert_snapshot;
//     use ratatui::{backend::TestBackend, Terminal};

//     #[test]
//     fn test_render() {
//         let app = App::default();
//         let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
//         terminal
//             .draw(|frame| frame.render_widget(&app, frame.area()))
//             .unwrap();

//         assert_snapshot!(terminal.backend());
//     }

//     #[test]
//     fn test_handle_key_events() {
//         let mut app = App::default();
//         let key_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
//         handle_key_events(key_event, &mut app).unwrap();
//         assert!(!app.running);

//         let mut app = App::default();
//         let key_event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
//         handle_key_events(key_event, &mut app).unwrap();
//         assert!(!app.running);

//         let key_event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
//         handle_key_events(key_event, &mut app).unwrap();
//         assert_eq!(app.counter, 1);

//         let key_event = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
//         handle_key_events(key_event, &mut app).unwrap();
//         assert_eq!(app.counter, 0);
//     }
// }
