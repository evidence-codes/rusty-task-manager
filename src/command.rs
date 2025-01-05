pub enum Command {
    Add(String),
    View,
    Complete(usize),
    Delete(usize),
    Save,
    Load,
    Exit,
}
