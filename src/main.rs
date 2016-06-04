extern crate yaml_rust;
extern crate tina;

use std::env;
use std::str;
use std::io::{BufRead, BufReader};
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

	let res = match wni_client.connect() {
		Ok(v) => v,
		Err(e) => {
			println!("{:?}", e);
			return;
		}
	};

	let h = format!("{:?}", (&res.headers).clone());
	let mut body = BufReader::new(res);

	println!("[Header] {}", h);

	loop {

		let mut buf = Vec::new();


		match body.read_until(b'\n', &mut buf) {

			Err(e) => {
				println!("[Error] {:?}", e);
				return;
			}
			Ok(size) => {
				if size == 0 {
					println!("[Connection] connection closed.");
					return;
				} else {
					match str::from_utf8(&buf) {
						Ok(r) => print!("[Body] {}", r),
						Err(_) => println!("[Body(Raw)] {:?}", buf)
					};
				}
			}
		};
	}
}
