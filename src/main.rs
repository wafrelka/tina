#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate csv;
extern crate tina;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;
extern crate slog_term;

mod config;

use std::io::stdout;
use std::env;
use std::sync::Arc;
use std::fs::OpenOptions;

use slog::{Drain, Logger, Discard, Duplicate};
use slog_scope::set_global_logger;
use slog_term::{PlainSyncDecorator, FullFormat};

use tina::*;
use config::*;

const ENV_VAR_NAME: &'static str = "TINA_CONF_PATH";
const DEFAULT_CONFIG_PATH: &'static str = "config/tina.yaml";

fn build_specific_logger(log_path: &Option<String>, duplication: bool, default: &Logger) -> Logger
{
	if let Some(ref path) = *log_path {

		let file = OpenOptions::new().append(true).create(true).open(path).unwrap();
		let drain = FullFormat::new(PlainSyncDecorator::new(file)).build();

		if duplication {
			Logger::root(Duplicate(drain, default.clone()).fuse(), o!())
		} else {
			Logger::root(drain.fuse(), o!())
		}

	} else {

		if duplication { default.clone() } else { Logger::root(Discard, o!()) }
	}
}

fn main()
{
	let cmd_args: Vec<String> = env::args().collect();
	let env_arg_string = env::var(ENV_VAR_NAME).ok();

	let cmd_arg = cmd_args.get(1).map(|s| s.as_str());
	let env_arg = env_arg_string.as_ref().map(|s| s.as_str());
	let conf_path = cmd_arg.or(env_arg).unwrap_or(DEFAULT_CONFIG_PATH);

	let conf = match Config::load_config(conf_path) {
		Err(err) => {
			println!("Error while loading config ({:?})", err);
			return;
		},
		Ok(c) => c
	};

	let stdout_drain = FullFormat::new(PlainSyncDecorator::new(stdout())).build();
	let stdout_logger = Logger::root(stdout_drain.fuse(), o!());

	let root_drain = stdout_logger.clone().filter_level(conf.log.log_level);
	let root_logger = Logger::root(root_drain.fuse(), o!());
	set_global_logger(root_logger).cancel_reset();

	let eew_logger = build_specific_logger(&conf.log.eew_log_path, conf.log.eew_stdout_log, &stdout_logger);
	let wni_logger = build_specific_logger(&conf.log.wni_log_path, conf.log.wni_stdout_log, &stdout_logger);

	let wni_client = WNIClient::new(conf.wni.id.clone(), conf.wni.password.clone(), Some(wni_logger));
	let mut socks: Vec<EEWSocket> = Vec::new();

	socks.push(EEWSocket::new(Logging::new(eew_logger)));
	info!("Enabled: EEW Logging");

	if conf.twitter.is_some() {
		let t = &conf.twitter.unwrap();
		let tw = Twitter::new(
			t.consumer_token.clone(), t.consumer_secret.clone(),
			t.access_token.clone(), t.access_secret.clone(), t.in_reply_to_enabled);
		if ! tw.is_valid() {
			warn!("Twitter: Invalid tokens");
		} else {
			socks.push(EEWSocket::new(tw));
			info!("Enabled: Twitter");
		}
	}

	loop {

		let mut connection = match wni_client.connect() {
			Ok(v) => v,
			Err(e) => {
				error!("ConnectionError: {:?}", e);
				continue;
			}
		};

		loop {

			let eew = match connection.wait_for_telegram(&conf.epicenter_dict, &conf.area_dict) {
				Err(e) => {
					error!("StreamingError: {:?}", e);
					break;
				},
				Ok(eew) => Arc::new(eew)
			};

			for s in socks.iter() {
				s.emit(eew.clone());
			}
		}
	}
}
