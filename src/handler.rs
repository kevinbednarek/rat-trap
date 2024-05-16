use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Char(char) => app.draw_char_guess(char),
        KeyCode::Enter => app.hint(),
        _ => {}
    }
    Ok(())
}
