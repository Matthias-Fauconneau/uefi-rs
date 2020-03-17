//! Text I/O.

mod input;
pub use self::input::{Input, Key, ScanCode, RawKey};

mod output;
pub use self::output::{Color, Output, OutputMode, OutputData};
