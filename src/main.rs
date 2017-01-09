extern crate yaml_rust;
extern crate csv;
extern crate tina;
extern crate log4rs;
#[macro_use] extern crate log;

mod config;

use std::env;
use std::sync::Arc;

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

	match setup_global_logger(conf.log_config) {
		Err(_) => {
			println!("Error while initializing log setting");
			return;
		},
		Ok(_) => {}
	};

	let wni_client = WNIClient::new(conf.wni_id.clone(), conf.wni_password.clone());
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
