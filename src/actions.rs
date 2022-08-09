use crossterm::event::KeyCode;

pub enum ScrollDirection {
    Up, // who doesn't love reinventing bools
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
    /// Scroll the output
    Scroll(ScrollDirection),
    /// Go to Help Screen
    Help,
}

impl TryFrom<crossterm::event::KeyEvent> for Actions {
    type Error = ();

    fn try_from(key: crossterm::event::KeyEvent) -> Result<Self, Self::Error> {
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
