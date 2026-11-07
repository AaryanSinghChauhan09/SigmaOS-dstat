//! SigmaOS-dstat — Versatile resource statistics model.

pub mod util;
pub mod options;

pub use util::{Dstat, StatSample};
pub use options::{DstatOptions, Mode};
