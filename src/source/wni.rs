use std::io::{BufRead, BufReader, Write};
use std::time::Duration;
use std::collections::HashMap;
use std::net::TcpStream;
use std::str;

use reqwest::Client;
use crypto::digest::Digest;
use crypto::md5::Md5;
use rand::{thread_rng, Rng};
use slog::{Logger, Discard};
use chrono::{DateTime, Utc, TimeZone};

use eew::EEW;
use parser::{parse_jma_format, JMAFormatParseError};

const CONNECTION_TIMEOUT_SECS: u64 = 3 * 60;
const DELAY_THRESHOLD_MS: i64 = 2000;
const DATE_FORMAT: &'static str = "%a, %d %b %Y %T%.6f UTC";
const X_WNI_TIME_FORMAT: &'static str = "%Y/%m/%d %T%.6f";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum WniError {
	Authentication,
	Network,
	ConnectionClosed,
	InvalidData,
	TooSlow,
	ParseError(JMAFormatParseError),
}

#[derive(Clone, Debug)]
pub struct Wni {
	wni_id: String,
	wni_terminal_id: String,
	wni_password: String,
	server_list_url: String,
	client: Client,
	logger: Logger,
}

fn to_header_date(utc: &DateTime<Utc>) -> String
{
	utc.format(DATE_FORMAT).to_string()
}

fn from_header_date(txt: &[u8]) -> Option<DateTime<Utc>>
{
	str::from_utf8(txt).ok().and_then(|s| Utc.datetime_from_str(s, DATE_FORMAT).ok())
}

fn to_wni_time(utc: &DateTime<Utc>) -> String
{
	utc.format(X_WNI_TIME_FORMAT).to_string()
}

fn from_wni_time(txt: &[u8]) -> Option<DateTime<Utc>>
{
	str::from_utf8(txt).ok().and_then(|s| Utc.datetime_from_str(s, X_WNI_TIME_FORMAT).ok())
}

impl Wni {

	pub fn new(wni_id: String, wni_terminal_id: String,
		wni_password: String, server_list_url: String, logger: Option<Logger>) -> Wni
	{
		Wni {
			wni_id: wni_id,
			wni_terminal_id: wni_terminal_id,
			wni_password: wni_password,
			server_list_url: server_list_url,
			client: Client::new(),
			logger: logger.unwrap_or(Logger::root(Discard, o!())),
		}
	}

	pub fn retrieve_server(&self) -> Result<String, WniError>
	{
		let mut resp = self.client.get(&self.server_list_url).send().map_err(|_| WniError::Network)?;

		if ! resp.status().is_success() {
			return Err(WniError::Network);
		}
		let text = resp.text().map_err(|_| WniError::Network)?;
		let servers: Vec<&str> = text.split('\n').filter(|&s| s != "").collect();
		let chosen = thread_rng().choose(&servers).ok_or(WniError::Network)?;

		Ok(chosen.to_string())
	}

	pub fn connect(&self) -> Result<WniConnection, WniError>
	{
		let server = self.retrieve_server()?;
		let conn = WniConnection::open(server,
			&self.wni_id, &self.wni_terminal_id, &self.wni_password, &self.logger)?;

		Ok(conn)
	}
}

#[derive(Debug)]
pub struct WniConnection<'a> {
	server: String,
	reader: BufReader<TcpStream>,
	logger: &'a Logger,
	too_slow: bool,
}

impl<'a> WniConnection<'a> {

	pub fn open<'b>(server: String, wni_id: &'b str, wni_terminal_id: &'b str,
		wni_password: &'b str, logger: &'a Logger) -> Result<WniConnection<'a>, WniError>
	{
		let stream = TcpStream::connect(&server).map_err(|_| WniError::Network)?;
		stream.set_nodelay(true).expect("set_nodelay call failed");
		stream.set_read_timeout(Some(Duration::from_secs(CONNECTION_TIMEOUT_SECS)))
			.expect("set_read_timeout call failed");
		stream.set_write_timeout(Some(Duration::from_secs(CONNECTION_TIMEOUT_SECS)))
			.expect("set_write_timeout call failed");

		let reader = BufReader::new(stream);

		let mut conn = WniConnection {
			server: server,
			reader: reader,
			logger: logger,
			too_slow: false,
		};

		conn.write_request(wni_id, wni_terminal_id, wni_password)?;
		let resp = conn.read_headers()?;

		if resp.iter().any(|h| h == b"X-WNI-Result: OK") {
			Ok(conn)
		} else {
			Err(WniError::Authentication)
		}
	}

	fn output_log(&mut self, data: &[u8])
	{
		// skip the last LF if needed (the behavior is different from String::trim_right_matches)
		let right_idx = match data.last() {
			Some(&b'\n') => data.len() - 1,
			_ => data.len(),
		};

		let mut formatted = String::new();

		for c in data[0..right_idx].iter() {
			if c.is_ascii() && *c != b'\\' {
				formatted.push(*c as char);
			} else {
				formatted.push_str(&format!("\\x{:02x}", c));
			}
		}

		slog_info!(self.logger, "[{}] {}", self.server, formatted);
	}

	fn read_until(&mut self, marker: u8) -> Result<Vec<u8>, WniError>
	{
		let mut buf = Vec::new();

		match self.reader.read_until(marker, &mut buf) {
			Err(_) => return Err(WniError::Network),
			Ok(0) => return Err(WniError::ConnectionClosed),
			Ok(_) => {}
		}

		self.output_log(&buf);

		if buf.last() != Some(&marker) {
			Err(WniError::ConnectionClosed)
		} else {
			Ok(buf)
		}
	}

	fn read_headers(&mut self) -> Result<Vec<Vec<u8>>, WniError>
	{
		let mut headers = Vec::new();

		loop {

			let mut buf = self.read_until(b'\n')?;
			buf.pop();
			if buf.is_empty() {
				break;
			}
			headers.push(buf);
		}

		Ok(headers)
	}

	fn write_response(&mut self) -> Result<(), WniError>
	{
		let now = Utc::now();
		let resp = format!("\
			HTTP/1.0 200 OK\n\
			Content-Type: application/fast-cast\n\
			Date: {}\n\
			Server: FastCaster/1.0.0 (Unix)\n\
			X-WNI-ID: Response\n\
			X-WNI-Protocol-Version: 2.1\n\
			X-WNI-Result: OK\n\
			X-WNI-Time: {}\n\n",
			to_header_date(&now),
			to_wni_time(&now)
		);

		let stream = self.reader.get_mut();
		stream.write(resp.as_bytes()).map_err(|_| WniError::Network)?;
		stream.flush().map_err(|_| WniError::Network)?;

		Ok(())
	}

	fn write_request(&mut self, wni_id: &str, wni_terminal_id: &str, wni_password: &str)
		-> Result<(), WniError>
	{
		let mut hasher = Md5::new();
		hasher.input(wni_password.as_bytes());
		let hashed_password = hasher.result_str();

		let now = Utc::now();
		let req = format!("\
			GET /login HTTP/1.0\n\
			Accept: */*\n\
			Accept-Language: ja\n\
			Cache-Control: no-cache\n\
			Date: {}\n\
			User-Agent: FastCaster/1.0 powered by Weathernews.\n\
			X-WNI-Account: {}\n\
			X-WNI-Application-Version: 2.4.2\n\
			X-WNI-Authentication-Method: MDB_MWS\n\
			X-WNI-ID: Login\n\
			X-WNI-Password: {}\n\
			X-WNI-Protocol-Version: 2.1\n\
			X-WNI-Terminal-ID: {}\n\
			X-WNI-Time: {}\n\n",
			to_header_date(&now),
			wni_id,
			hashed_password,
			wni_terminal_id,
			to_wni_time(&now)
		);

		let stream = self.reader.get_mut();
		stream.write(req.as_bytes()).map_err(|_| WniError::Network)?;
		stream.flush().map_err(|_| WniError::Network)?;

		Ok(())
	}

	pub fn wait_for_telegram(&mut self,
		epicenter_dict: &HashMap<[u8; 3], String>,
		area_dict: &HashMap<[u8; 3], String>) -> Result<EEW, WniError>
	{
		if self.too_slow {
			return Err(WniError::TooSlow);
		}

		loop {

			let headers = self.read_headers()?;

			if headers.iter().any(|h| h == b"X-WNI-ID: Data") {

				let date_part = headers.iter()
					.find(|h| h.starts_with(b"Date: "))
					.and_then(|s| from_header_date(&s[6..]));
				let x_wni_time_part = headers.iter()
					.find(|h| h.starts_with(b"X-WNI-Time: "))
					.and_then(|s| from_wni_time(&s[12..]));

				let delta_ms = match (date_part, x_wni_time_part) {
					(Some(date), Some(x_wni_time)) =>
						Some(date.signed_duration_since(x_wni_time).num_milliseconds()),
					_ => None,
				};

				match delta_ms {
					Some(x) if x > DELAY_THRESHOLD_MS => {
						self.too_slow = true;
						info!("[{}] delay: {} (too slow)", self.server, x);
					},
					Some(x) => {
						debug!("[{}] delay: {}", self.server, x);
					},
					None => {
						warn!("[{}] arrival delay parse error", self.server);
					}
				}

				break;

			} else if ! headers.iter().any(|h| h == b"X-WNI-ID: Keep-Alive") {
				return Err(WniError::InvalidData);
			}

			self.write_response()?;
		}

		let buffer = self.read_until(b'\x03')?;

		let left = buffer.iter().rposition(|x| *x == b'\x02').ok_or(WniError::InvalidData)? + 2;
		let right = buffer.len() - 2;

		if left >= right {
			return Err(WniError::InvalidData);
		}

		let raw_data = &buffer[left..right];
		let eew = parse_jma_format(raw_data, epicenter_dict, area_dict)
			.map_err(|e| WniError::ParseError(e))?;

		self.write_response()?;

		Ok(eew)
	}

	pub fn server(&self) -> &str
	{
		&self.server
	}
}
