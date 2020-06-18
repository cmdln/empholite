#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Mode {
    View,
    Edit,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::View
    }
}
