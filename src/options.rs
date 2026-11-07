//! Options model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Default,
    Csv,
    Output(String),
    Top,
    Nfs,
}

#[derive(Debug, Clone)]
pub struct DstatOptions {
    pub mode: Mode,
    pub interval: u64,
    pub count: usize,
}

impl Default for DstatOptions {
    fn default() -> Self { Self { mode: Mode::Default, interval: 1, count: 0 } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(DstatOptions::default().mode, Mode::Default); }
}
