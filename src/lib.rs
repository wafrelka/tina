extern crate oauthcli;
extern crate url;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate chrono;
extern crate md5;
extern crate rand;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;
extern crate reqwest;

macro_rules! write_unwrap {
	($dst:expr, $($arg:tt)*) => (write!($dst, $($arg)*).unwrap())
}

mod eew;
mod parser;
mod collections;
mod source;
mod destination;
mod translator;
mod moderator;
mod condition;

pub use self::collections::*;
pub use self::eew::*;
pub use self::parser::*;
pub use self::source::Wni;
pub use self::destination::{Twitter, Logging, Slack, Router, Routing};
pub use self::translator::{ja_format_eew_oneline, format_eew_full};
pub use self::moderator::Moderator;
pub use self::condition::{Condition, ConstantCondition, TRUE_CONDITION, FALSE_CONDITION,
	DisjunctiveCondition, ValueCondition};
