use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Char(char) => {
            match app.game_result {
                None => app.draw_char_guess(char),
                _ => {}
            }
        },
        KeyCode::Enter => app.hint(),
        _ => {}
    }
    Ok(())
}
