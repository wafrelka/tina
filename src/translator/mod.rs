mod japanese;
mod general;

pub use self::japanese::format_eew_oneline as ja_format_eew_oneline;
pub use self::japanese::format_eew_short as ja_format_eew_short;
pub use self::general::format_eew_full;
