extern crate yaml_rust;
extern crate tina;

use std::env;
use tina::*;

fn main()
{
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		return;
	}

	let tweet = args[1].clone();

	let consumer_key = "xxxxxxxxxx".to_string();
	let consumer_secret = "0000000000".to_string();
	let access_key = "1111111111-yyyyyyyyyyyyyyyyyyyy".to_string();
	let access_secret = "zzzzzzzzzzzzzzzzzzzz".to_string();

	let tc = TwitterClient::new(consumer_key, consumer_secret, access_key, access_secret);

	if ! tc.is_valid() {
		println!("Invalid token.");
		return;
	}

	let res = tc.update_status(tweet);

	println!("{:?}", res);
}
