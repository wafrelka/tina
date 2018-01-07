use std::io::{Read, BufRead, BufReader};
use std::time::Duration;
use std::collections::HashMap;
use std::ascii::AsciiExt;

use hyper::Client;
use hyper::client::Response;
use hyper::header::Headers;
use crypto::digest::Digest;
use crypto::md5::Md5;
use rand::{thread_rng, Rng};
use slog::{Logger, Discard};

use eew::EEW;
use parser::{parse_jma_format, JMAFormatParseError};


header! { (XWniAccount, "X-WNI-Account") => [String] }
header! { (XWniPassword, "X-WNI-Password") => [String] }
header! { (XWniResult, "X-WNI-Result") => [String] }

const SERVER_LIST_URL: &'static str = "http://lst10s-sp.wni.co.jp/server_list.txt";
const LOGIN_PATH: &'static str = "/login";
const TIMEOUT_SECS: u64 = 3 * 60;
const ADDITIONAL_CAPACITY_FOR_HEX_ENCODING: usize = 256;
const DEFAULT_CAPACITY_FOR_TELEGRAM_BUFFER: usize = 2048;

#[derive(Debug, Clone)]
pub enum WniError {
	Authentication,
	Network,
	ConnectionClosed,
	InvalidData,
	ParseError(JMAFormatParseError)
}

pub struct WniClient {
	wni_id: String,
	wni_password: String,
	client: Client,
	logger: Logger,
}

fn from_data_to_string(raw: &[u8]) -> String
{
	// reduce reallocation
	let mut s = String::with_capacity(raw.len() + ADDITIONAL_CAPACITY_FOR_HEX_ENCODING);

	for c in raw.iter() {
		if c.is_ascii() && *c != b'\\' {
			s.push(*c as char);
		} else {
			s.push_str(&format!("\\x{:02x}", c));
		}
	}

	s
}

impl WniClient {

	pub fn new(wni_id: String, wni_password: String, logger: Option<Logger>) -> WniClient
	{
		let mut client = Client::new();
		client.set_read_timeout(Some(Duration::from_secs(TIMEOUT_SECS)));

		WniClient {
			wni_id: wni_id,
			wni_password: wni_password,
			client: client,
			logger: logger.unwrap_or(Logger::root(Discard, o!()))
		}
	}

	pub fn retrieve_server(&self) -> Result<String, WniError>
	{
		let mut res = try!(self.client.get(SERVER_LIST_URL).send().map_err(|_| WniError::Network));
		let mut body = String::new();
		try!(res.read_to_string(&mut body).map_err(|_| WniError::Network));

		let servers: Vec<&str> = body.split('\n').filter(|&s| s != "").collect();

		return thread_rng().choose(&servers).ok_or(WniError::Network).map(|s| s.to_string());
	}

	pub fn connect(&self) -> Result<WniConnection, WniError>
	{
		let server = try!(self.retrieve_server());
		let url = format!("http://{}{}", server, LOGIN_PATH);

		let mut hasher = Md5::new();
		hasher.input(self.wni_password.as_bytes());
		let hashed = hasher.result_str();

		let mut headers = Headers::new();
		headers.set(XWniAccount(self.wni_id.clone()));
		headers.set(XWniPassword(hashed));

		let res = try!(self.client.get(&url).headers(headers).send().map_err(|_| WniError::Network));

		if res.headers.get::<XWniResult>() != Some(&XWniResult("OK".to_owned())) {
			Err(WniError::Authentication)
		} else {
			Ok(WniConnection::new(res, &self.logger))
		}
	}
}

pub struct WniConnection<'a> {
	reader: BufReader<Response>,
	logger: &'a Logger,
}

impl<'a> WniConnection<'a> {

	pub fn new(response: Response, logger: &Logger) -> WniConnection
	{
		let reader = BufReader::new(response);
		WniConnection { reader: reader, logger: logger }
	}

	fn read_until(&mut self, byte: u8) -> Result<Vec<u8>, WniError>
	{
		// reduce reallocation
		let mut buf = Vec::with_capacity(DEFAULT_CAPACITY_FOR_TELEGRAM_BUFFER);

		match self.reader.read_until(byte, &mut buf) {
			Err(_) => Err(WniError::Network),
			Ok(0) => Err(WniError::ConnectionClosed),
			Ok(_) =>
				if buf.last().map(|v| *v) != Some(byte) {
					Err(WniError::ConnectionClosed)
				} else {
					Ok(buf)
				}
		}
	}

	fn output_log(&self, buf: &Vec<u8>)
	{
		// remove the last LF if needed (the behavior is different from String::trim_right_matches)
		let trailing_lf = buf.last() == Some(b'\n').as_ref();
		let trimmed = if trailing_lf { &buf[0..(buf.len() - 1)] } else { &buf };

		slog_info!(self.logger, "{}", from_data_to_string(trimmed));
	}

	pub fn wait_for_telegram(&mut self,
		epicenter_dict: &HashMap<[u8; 3], String>,
		area_dict: &HashMap<[u8; 3], String>) -> Result<EEW, WniError>
	{
		loop {
			let buffer = try!(self.read_until(b'\n'));
			self.output_log(&buffer);
			if buffer == b"\x01\n" {
				break;
			}
		}

		let buffer = try!(self.read_until(b'\x03'));
		self.output_log(&buffer);

		let left = try!(buffer.iter().rposition(|&x| x == b'\x02').ok_or(WniError::InvalidData)) + 2;
		let right = buffer.len() - 2;

		if left >= right {
			return Err(WniError::InvalidData);
		}

		let raw_data = &buffer[left..right];
		let eew = try!(parse_jma_format(raw_data, epicenter_dict, area_dict)
			.map_err(|e| WniError::ParseError(e)));

		return Ok(eew);
	}
}
