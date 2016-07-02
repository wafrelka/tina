use std::io::{Read, BufRead, BufReader};
use std::time::Duration;
use std::collections::HashMap;

use hyper::Client;
use hyper::client::Response;
use hyper::header::Headers;
use crypto::digest::Digest;
use crypto::md5::Md5;
use rand::{thread_rng, Rng};

use eew::EEW;
use parser::parse_jma_format;


header! { (XWNIAccount, "X-WNI-Account") => [String] }
header! { (XWNIPassword, "X-WNI-Password") => [String] }
header! { (XWNIResult, "X-WNI-Result") => [String] }

const SERVER_LIST_URL: &'static str = "http://lst10s-sp.wni.co.jp/server_list.txt";
const LOGIN_PATH: &'static str = "/login";
const TIMEOUT_SECS: u64 = 3 * 60;

#[derive(Debug)]
pub enum WNIError {
	Authentication,
	Network,
	ConnectionClosed,
	InvalidData,
}

pub struct WNIClient {
	wni_id: String,
	wni_password: String,
	client: Client
}

impl WNIClient {

	pub fn new(wni_id: String, wni_password: String) -> WNIClient
	{
		let mut client = Client::new();
		client.set_read_timeout(Some(Duration::from_secs(TIMEOUT_SECS)));

		WNIClient {
			wni_id: wni_id,
			wni_password: wni_password,
			client: client
		}
	}

	pub fn retrieve_server(&self) -> Result<String, WNIError>
	{
		let mut res = try!(self.client.get(SERVER_LIST_URL).send().map_err(|_| WNIError::Network));
		let mut body = String::new();

		try!(res.read_to_string(&mut body).map_err(|_| WNIError::Network));

		let servers: Vec<&str> = body.split('\n').filter(|&s| s != "").collect();

		return thread_rng().choose(&servers).ok_or(WNIError::Network).map(|s| s.to_string());
	}

	pub fn connect(&self) -> Result<WNIConnection, WNIError>
	{
		let server = try!(self.retrieve_server());
		let url = format!("http://{}{}", server, LOGIN_PATH);

		let mut hasher = Md5::new();
		hasher.input(self.wni_password.as_bytes());
		let hashed = hasher.result_str();

		let mut headers = Headers::new();
		headers.set(XWNIAccount(self.wni_id.clone()));
		headers.set(XWNIPassword(hashed));

		let res = try!(self.client.get(&url).headers(headers).send().map_err(|_| WNIError::Network));

		if res.headers.get::<XWNIResult>() != Some(&XWNIResult("OK".to_owned())) {
			return Err(WNIError::Authentication);
		}

		return Ok(WNIConnection::new(res));
	}
}

pub struct WNIConnection {
	reader: BufReader<Response>
}

impl WNIConnection {

	pub fn new(response: Response) -> WNIConnection
	{
		let reader = BufReader::new(response);
		WNIConnection { reader: reader }
	}

	pub fn wait_for_telegram(&mut self,
		epicenter_dict: &HashMap<[u8; 3], String>,
		area_dict: &HashMap<[u8; 3], String>) -> Result<EEW, WNIError>
	{
		let mut buffer = vec! {};

		let size = try!(self.reader.read_until(b'\x03', &mut buffer)
			.map_err(|_| WNIError::Network));

		if size == 0 {
			return Err(WNIError::ConnectionClosed);
		}

		println!("Received: {}", String::from_utf8_lossy(&buffer));

		let left = try!(buffer.iter().rposition(|&x| x == b'\x02')
			.ok_or(WNIError::InvalidData)) + 2;
		let right = buffer.len() - 2;

		if left >= right {
			return Err(WNIError::InvalidData);
		}

		let raw_data = &buffer[left..right];
		let eew = try!(parse_jma_format(raw_data, epicenter_dict, area_dict)
			.map_err(|_| WNIError::InvalidData));

		return Ok(eew);
	}
}
