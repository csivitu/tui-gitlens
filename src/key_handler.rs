use ratatui::crossterm::event::{self, poll, KeyCode, KeyEventKind};
use std::{error::Error, time::Duration};

use crate::app_state;

pub fn read_key(app: &mut app_state::State) -> Result<(), Box<dyn Error>> {
    if poll(Duration::from_millis(app.delay))? {
        match event::read() {
            Ok(event::Event::Key(key)) => {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    app.running = false;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('n') {
                    app.num_widgets += 1;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('N') {
                    app.num_widgets -= 1;
                }
            }
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
    }
    return Ok(());
}
