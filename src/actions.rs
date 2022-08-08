use crossterm::event::KeyCode;

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
}

impl TryFrom<crossterm::event::KeyEvent> for Actions {
    type Error = ();

    fn try_from(key: crossterm::event::KeyEvent) -> Result<Self, Self::Error> {
        let action = match key.code {
            KeyCode::Char('q') => Actions::Exit,
            KeyCode::Up => Actions::Previous,
            KeyCode::Down => Actions::Next,
            _ => return Err(()),
        };

        Ok(action)
    }
}
