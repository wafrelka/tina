extern crate url;
extern crate hyper;
extern crate crypto;

use std::io::{Read};
use self::hyper::Client;
use self::hyper::client::Response;
use self::hyper::header::Headers;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;


header! { (XWNIAccount, "X-WNI-Account") => [String] }
header! { (XWNIPassword, "X-WNI-Password") => [String] }

const SERVER_LIST_URL: &'static str = "http://lst10s-sp.wni.co.jp/server_list.txt";
const LOGIN_PATH: &'static str = "/login";

#[derive(Debug)]
pub enum WNIError {
	Authentication,
	Network
}

pub struct WNIClient {
	wni_id: String,
	wni_password: String,
	client: Client
}

impl WNIClient {

	pub fn new(wni_id: String, wni_password: String) -> WNIClient
	{
		WNIClient {
			wni_id: wni_id,
			wni_password: wni_password,
			client: Client::new()
		}
	}

	pub fn retrieve_server(&self) -> Result<String, WNIError>
	{
		let mut res = try!(self.client.get(SERVER_LIST_URL).send().map_err(|_| WNIError::Network));
		let mut body = String::new();

		try!(res.read_to_string(&mut body).map_err(|_| WNIError::Network));
		return body.split('\n').next().ok_or(WNIError::Network).map(|s| s.to_string());
	}

	pub fn connect(&self) -> Result<Response, WNIError>
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

		return Ok(res);
	}
}
