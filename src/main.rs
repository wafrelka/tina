extern crate yaml_rust;
extern crate csv;
extern crate tina;

mod config;

use std::env;

use tina::*;
use config::*;


fn main()
{
	let args: Vec<String> = env::args().collect();

	let conf_path = match args.len() >= 2 {
		true => args[1].as_str(),
		false => "tina.yaml",
	};

	let conf = match Config::load_config(conf_path) {
		Err(err) => {
			println!("Error while loading config ({:?})", err);
			return;
		},
		Ok(c) => c
	};

	let tw_func = |_: &[EEW], latest: &EEW| {
		match ja_format_eew_short(latest, None) {
			Some(v) => Some(Box::new(v)),
			None => None
		}
	};

	let stdout_func = |_: &[EEW], latest: &EEW| {
		Some(Box::new(format_eew_full(latest)))
	};

	let wni_client = WNIClient::new(conf.wni_id.clone(), conf.wni_password.clone());
	let mut dests: Vec<Box<Emit>> = Vec::new();

	if conf.twitter.is_some() {
		let t = &conf.twitter.unwrap();
		let tc = Box::new(TwitterClient::new(
			t.consumer_token.clone(), t.consumer_secret.clone(),
			t.access_token.clone(), t.access_secret.clone()));
		let te = Box::new(Emitter::new(tc, &tw_func));
		dests.push(te);
		println!("Use: Twitter");
	}

	let sl = Box::new(StdoutLogger::new());
	let se = Box::new(Emitter::new(sl, &stdout_func));
	dests.push(se);
	println!("Use: Stdout");

	let mut buffer = EEWBuffer::new();

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
				Ok(eew) => eew
			};

			if let Some(eews) = buffer.append(&eew) {
				for d in dests.iter() {
					d.emit(&eews, &eew);
				}
			}
		}
	}
}
