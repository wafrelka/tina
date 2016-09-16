extern crate yaml_rust;
extern crate csv;
extern crate tina;

mod config;

use std::env;
use std::sync::Arc;

use tina::*;
use config::*;


fn main()
{
	let args: Vec<String> = env::args().collect();

	let conf_path = args.get(1).map(|s| s.as_str()).unwrap_or("tina.yaml");

	let conf = match Config::load_config(conf_path) {
		Err(err) => {
			println!("Error while loading config ({:?})", err);
			return;
		},
		Ok(c) => c
	};

	let tw_fn = |eews: &[Arc<EEW>], latest: Arc<EEW>, tw: &mut TwitterClient| {

		if let Some(out) = ja_format_eew_short(&latest, eews.iter().rev().nth(1).map(|e| e.as_ref())) {
			tw.output(&out, None);
		}
	};

	let stdout_fn = |_: &[Arc<EEW>], latest: Arc<EEW>, stdout_logger: &mut StdoutLogger| {

		let out = format_eew_full(&latest);
		stdout_logger.output(&out);
	};

	let wni_client = WNIClient::new(conf.wni_id.clone(), conf.wni_password.clone());
	let mut cons: Vec<Connector> = Vec::new();

	cons.push(Connector::new(stdout_fn, StdoutLogger::new()));
	println!("Use: Stdout");

	if conf.twitter.is_some() {
		let t = &conf.twitter.unwrap();
		let tc = TwitterClient::new(
			t.consumer_token.clone(), t.consumer_secret.clone(),
			t.access_token.clone(), t.access_secret.clone());
		cons.push(Connector::new(tw_fn, tc));
		println!("Use: Twitter");
	}

	loop {

		let mut connection = match wni_client.connect() {
			Ok(v) => v,
			Err(e) => {
				println!("ConnectionError: {:?}", e);
				continue;
			}
		};

		loop {

			let eew = match connection.wait_for_telegram(&conf.epicenter_dict, &conf.area_dict) {
				Err(e) => {
					println!("StreamingError: {:?}", e);
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
