extern crate yaml_rust;
extern crate csv;
extern crate tina;

use std::env;
use std::collections::HashMap;

use tina::*;


fn load_code_dict(path: &str) -> Option<HashMap<[u8; 3], String>>
{
	let mut reader = match csv::Reader::from_file(path) {
		Ok(v) => v,
		Err(_) => return None
	};

	let mut dict = HashMap::new();

	for record in reader.decode() {

		let (code, name): (String, String) = match record {
			Ok(v) => v,
			Err(_) => return None
		};

		let mut encoded = [0; 3];
		let bytes = code.as_bytes();

		if bytes.len() != encoded.len() {
			return None;
		}

		for i in 0..(encoded.len()) {
			encoded[i] = bytes[i];
		}

		dict.insert(encoded, name);
	}

	return Some(dict);
}

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() < 9 {
		println!("usage: {} <epicenter_path> <area_path> <wni_id> <wni_password> \
			<tw_con_token> <tw_con_sec> <tw_ac_token> <tw_ac_sec>", args[0]);
		return;
	}

	let epicenter_dict_path = args[1].clone();
	let area_dict_path = args[2].clone();
	let wni_id = args[3].clone();
	let wni_password = args[4].clone();
	let tw_consumer_token = args[5].clone();
	let tw_consumer_secret = args[6].clone();
	let tw_access_token = args[7].clone();
	let tw_access_secret = args[8].clone();

	let epicenter_dict = match load_code_dict(&epicenter_dict_path) {
		Some(v) => v,
		None => {
			println!("cannot load epicenter dict");
			return;
		}
	};

	let area_dict = match load_code_dict(&area_dict_path) {
		Some(v) => v,
		None => {
			println!("cannot load area dict");
			return;
		}
	};

	let wni_client = WNIClient::new(wni_id.to_string(), wni_password.to_string());

	let tc = Box::new(TwitterClient::new(tw_consumer_token, tw_consumer_secret, tw_access_token, tw_access_secret));
	let tf = move |eews: &[EEW]| {
		let eew = eews.last().unwrap();
		match ja_format_eew_short(&eew) {
			Some(v) => Some(Box::new(v)),
			None => None
		}
	};
	let te = Emitter::new(tc, &tf);

	let sl = Box::new(StdoutLogger::new());
	let sf = move |eews: &[EEW]| {
		let eew = eews.last().unwrap();
		Some(Box::new(ja_format_eew_detailed(&eew)))
	};
	let se = Emitter::new(sl, &sf);

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

			let eew = match connection.wait_for_telegram(&epicenter_dict, &area_dict) {
				Err(e) => {
					println!("StreamingError: {:?}", e);
					break;
				},
				Ok(eew) => eew
			};

			let eews = buffer.append(&eew);

			se.emit(&eews);
			te.emit(&eews);
		}
	}
}
