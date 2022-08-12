use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::info;

/// What direction to scroll in
pub enum ScrollDirection {
    /// Scroll up
    Up,
    /// Scroll down
    Down,
}

/// Actions to control bang
pub enum Actions {
    /// Exit bang
    Exit,
    /// Kill command
    Kill,
    /// Restart command
    Restart,
    /// Select previous command
    Previous,
    /// Select next command
    Next,
    /// Scroll the output. Direction and multiple
    Scroll(ScrollDirection, usize),
    /// Go to Help Screen
    Help,
}

impl TryFrom<KeyEvent> for Actions {
    type Error = ();

    fn try_from(key: KeyEvent) -> Result<Self, Self::Error> {
        if key.modifiers == KeyModifiers::SHIFT {
            let action = match key.code {
                KeyCode::Up => Actions::Scroll(ScrollDirection::Up, 1),
                KeyCode::Down => Actions::Scroll(ScrollDirection::Down, 1),
                _ => return Err(()),
            };
            return Ok(action);
        } else if key.modifiers == KeyModifiers::CONTROL {
            let action = match key.code {
                KeyCode::Up => Actions::Scroll(ScrollDirection::Up, 10),
                KeyCode::Down => Actions::Scroll(ScrollDirection::Down, 10),
                _ => return Err(()),
            };
            return Ok(action);
        }

        let action = match key.code {
            KeyCode::Char('q') => Actions::Exit,
            KeyCode::Char('r') => Actions::Restart,
            KeyCode::Char('k') => Actions::Kill,
            KeyCode::Char('?') => Actions::Help,

            KeyCode::Up => Actions::Previous,
            KeyCode::Down => Actions::Next,
            _ => return Err(()),
        };

        Ok(action)
    }
}
