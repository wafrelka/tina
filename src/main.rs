extern crate yaml_rust;
extern crate tina;

use std::env;
use tina::*;

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() < 3 {
		println!("usage: {} <wni_id> <wni_password>", args[0]);
		return;
	}

	let wni_id = args[1].clone();
	let wni_password = args[2].clone();

	let wni_client = WNIClient::new(wni_id.to_string(), wni_password.to_string());

	loop {

		let mut connection = match wni_client.connect() {
			Ok(v) => v,
			Err(e) => {
				println!("ConnectionError: {:?}", e);
				return;
			}
		};

		let eew = match connection.wait_for_telegram() {
			Err(e) => {
				println!("StreamingError: {:?}", e);
				continue;
			},
			Ok(eew) => eew
		};

		println!("EEW: {:?}", eew);
	}
}
