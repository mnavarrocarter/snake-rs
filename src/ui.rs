use std::time::Duration;
use crate::game::Game;
use crate::command::Command;

/// An interface for types that can implement the Game UI
pub trait UI {
    /// Initializes the UI
    fn init(&mut self, game: &Game);
    /// Renders one UI cycle
    fn render(&mut self, game: &Game);
    /// Restores the UI (closes windows, etc)
    fn shutdown(&mut self, game: &Game);
    /// Gets a game command from the UI
    fn get_command(&self, wait_for: Duration) -> Option<Command>;
}