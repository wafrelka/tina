extern crate oauthcli;
extern crate url;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate chrono;
extern crate crypto;
extern crate rand;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;

macro_rules! write_unwrap {
	($dst:expr, $($arg:tt)*) => (write!($dst, $($arg)*).unwrap())
}

mod eew;
mod parser;
mod eew_socket;
mod collections;
mod source;
mod destination;
mod translator;
mod moderator;
mod condition;

pub use self::collections::*;
pub use self::eew::*;
pub use self::parser::*;
pub use self::eew_socket::EEWSocket;
pub use self::source::{WNIClient};
pub use self::destination::{Twitter, Logging, Slack};
pub use self::translator::{ja_format_eew_short, format_eew_full};
pub use self::moderator::Moderator;
pub use self::condition::{Condition, ConstantCondition, TRUE_CONDITION, FALSE_CONDITION,
	DisjunctiveCondition, ValueCondition};
