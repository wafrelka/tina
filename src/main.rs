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

	match setup_logging(conf.log_config) {
		Err(_) => {
			println!("Error while initializing log setting");
			return;
		},
		Ok(_) => {}
	};

	let tw_fn = |eews: &[Arc<EEW>], latest: Arc<EEW>,
		state: &mut (TwitterClient, LimitedQueue<(String, u64)>, bool)| {

		let (ref tw, ref mut q, reply_enabled) = *state;

		let out = match ja_format_eew_short(&latest, eews.iter().rev().nth(1).map(|e| e.as_ref())) {
			Some(out) => out,
			None => return
		};

		let prev_tw_id_opt = match reply_enabled {
			true => q.iter().find(|x| x.0 == latest.id).map(|x| x.1),
			false => None
		};

		match tw.update_status(&out, prev_tw_id_opt) {

			Ok(tw_id) => {

				if prev_tw_id_opt == None {
					q.push((latest.id.clone(), tw_id));
				} else {
					q.iter_mut().find(|x| x.0 == latest.id).unwrap().1 = tw_id;
				}
			},

			Err(e) => {
				error!("TwitterError: {:?}", e);
			}
		}
	};

	let log_fn = |_: &[Arc<EEW>], latest: Arc<EEW>, lw: &mut LoggingWrapper| {

		let out = format_eew_full(&latest);
		lw.output(&out);
	};

	let wni_client = WNIClient::new(conf.wni_id.clone(), conf.wni_password.clone());
	let mut cons: Vec<Connector> = Vec::new();

	cons.push(Connector::new(log_fn, LoggingWrapper::new()));
	info!("Enabled: EEW Logging");

	if conf.twitter.is_some() {
		let t = &conf.twitter.unwrap();
		let tc = TwitterClient::new(
			t.consumer_token.clone(), t.consumer_secret.clone(),
			t.access_token.clone(), t.access_secret.clone());
		let q = LimitedQueue::with_allocation(16);
		cons.push(Connector::new(tw_fn, (tc, q, t.in_reply_to_enabled)));
		info!("Enabled: Twitter");
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

			for e in cons.iter() {
				e.emit(eew.clone());
			}
		}
	}
}
