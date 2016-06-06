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

fn print_eew(eew: &EEW)
{
	println!("[EEW: {} - {}]", eew.id, eew.number);
	println!("source: {:?}, kind: {:?}, issued_at: {:?}, occurred_at: {:?}, status: {:?}",
		eew.source, eew.kind, eew.issued_at, eew.occurred_at, eew.status);

	if let EEWDetail::Full(ref detail) = eew.detail {

		println!("issue_pattern: {:?}, epicenter_name: {}, epicenter: {:?}, depth: {:?}, \
			magnitude: {:?}, maximum_intensity: {:?}, epicenter_accuracy: {:?}, \
			depth_accuracy: {:?}, magnitude_accuracy: {:?}, epicenter_caterogy: {:?} \
			warning_status: {:?}, intensity_change: {:?}, change_reason: {:?}",
			detail.issue_pattern, detail.epicenter_name, detail.epicenter, detail.depth,
			detail.magnitude, detail.maximum_intensity, detail.epicenter_accuracy,
			detail.depth_accuracy, detail.magnitude_accuracy, detail.epicenter_caterogy,
			detail.warning_status, detail.intensity_change, detail.change_reason);

		for ref area in &detail.area_info {

			println!("area_name: {}, minimum_intensity: {:?}, maximum_intensity: {:?}, \
				reached_at: {:?}, warning_status: {:?}, wave_status: {:?}",
				area.area_name, area.minimum_intensity, area.maximum_intensity,
				area.reached_at, area.warning_status, area.wave_status);
		}
	}
}

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() < 5 {
		println!("usage: {} <epicenter_path> <area_path> <wni_id> <wni_password>", args[0]);
		return;
	}

	let epicenter_dict_path = args[1].clone();
	let area_dict_path = args[2].clone();
	let wni_id = args[3].clone();
	let wni_password = args[4].clone();

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

	loop {

		let mut connection = match wni_client.connect() {
			Ok(v) => v,
			Err(e) => {
				println!("ConnectionError: {:?}", e);
				continue;
			}
		};

		let eew = match connection.wait_for_telegram(&epicenter_dict, &area_dict) {
			Err(e) => {
				println!("StreamingError: {:?}", e);
				continue;
			},
			Ok(eew) => eew
		};

		print_eew(&eew);
	}
}
