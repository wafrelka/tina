#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate csv;
extern crate tina;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;
extern crate slog_term;

mod config;

use std::env;
use std::sync::Arc;
use slog::{Drain, LevelFilter};
use slog_scope::set_global_logger;
use slog_term::{PlainSyncDecorator, FullFormat};

use tina::*;
use config::*;

const ENV_VAR_NAME: &'static str = "TINA_CONF_PATH";
const DEFAULT_CONFIG_PATH: &'static str = "config/tina.yaml";


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

	let plain = PlainSyncDecorator::new(std::io::stdout());
	let drain = FullFormat::new(plain).build();
	let stdout_logger = slog::Logger::root(drain.fuse(), o!());

	let filter = LevelFilter::new(stdout_logger.new(o!()), conf.log.log_level);
	let root_logger = slog::Logger::root(filter.fuse(), o!());
	set_global_logger(root_logger).cancel_reset();

	let wni_client = WNIClient::new(conf.wni.id.clone(), conf.wni.password.clone());
	let mut socks: Vec<EEWSocket> = Vec::new();

	socks.push(EEWSocket::new(Logging::new()));
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
