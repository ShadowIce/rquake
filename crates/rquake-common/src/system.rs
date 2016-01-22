
pub trait Window {
    fn show_window(&self);
    fn is_running(&self) -> bool;
    fn handle_message(&mut self);
}