use std::io::Read;

use oauthcli::SignatureMethod::HmacSha1;
use oauthcli::{timestamp, nonce, authorization_header};
use url::Url;
use url::form_urlencoded::Serializer;
use hyper::{Client, Error};
use hyper::client::Response;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::header::{Headers, Authorization, ContentType};

use destination::Destination;
use emitter::Emitter;


pub struct TwitterClient {
	consumer_key: String,
	consumer_secret: String,
	access_key: String,
	access_secret: String,
	client: Client
}

#[derive(Debug)]
pub enum StatusUpdateError {
	Duplicated,
	RateLimitExceeded,
	Invalid,
	Network,
	Unauthorized,
	Unknown
}

impl TwitterClient {

	pub fn new(consumer_key: String, consumer_secret: String, access_key: String, access_secret: String) -> TwitterClient
	{
		TwitterClient {
			consumer_key: consumer_key,
			consumer_secret: consumer_secret,
			access_key: access_key,
			access_secret: access_secret,
			client: Client::new()
		}
	}

	pub fn is_valid(&self) -> bool
	{
		// TODO: implement token validation
		return true;
	}

	pub fn update_status(&self, message: String)
	 -> Result<(), StatusUpdateError>
	{
		let api_url = "https://api.twitter.com/1.1/statuses/update.json";
		let args = vec![("status".to_string(), message)];
		let res = self.request(Method::Post, api_url, args);

		match res {
			Ok(mut res) => {
				match res.status {

					StatusCode::Forbidden => {

						let mut body = String::new();

						if let Err(_) = res.read_to_string(&mut body) {
							return Err(StatusUpdateError::Unknown);
						}

						if body.contains("140") {
							return Err(StatusUpdateError::Invalid);
						} else if body.contains("duplicate") {
							return Err(StatusUpdateError::Duplicated);
						}

						return Err(StatusUpdateError::Unknown);

					},

					StatusCode::Ok => return Ok(()),
					StatusCode::TooManyRequests =>
						return Err(StatusUpdateError::RateLimitExceeded),
					StatusCode::Unauthorized =>
						return Err(StatusUpdateError::Unauthorized),
					_ => return Err(StatusUpdateError::Unknown)
				}
			},
			Err(_) => return Err(StatusUpdateError::Network)
		}
	}

	fn request(&self, method: Method, api_url: &str, args: Vec<(String, String)>)
	 -> Result<Response, Error>
	{
		match method {

			Method::Get => {

				let mut headers = Headers::new();
				let oauth_header = self.construct_oauth_header("GET", api_url, args);
				headers.set(Authorization(oauth_header));

				let result = self.client.get(api_url).headers(headers).send();

				return result;

			},

			Method::Post => {

				let content_type = ContentType("application/x-www-form-urlencoded".parse().unwrap());

				let mut args_serializer = Serializer::new(String::new());
				for &(ref k, ref v) in &args {
					args_serializer.append_pair(k, v);
				}
				let args_serialized = args_serializer.finish();

				let mut headers = Headers::new();
				let oauth_header = self.construct_oauth_header("POST", api_url, args);
				headers.set(Authorization(oauth_header));
				headers.set(content_type);

				let result = self.client.post(api_url).body(&args_serialized).headers(headers).send();

				return result;

			},

			_ => {
				panic!();
			}
		}
	}

	fn construct_oauth_header(&self, method: &str, api_url: &str, args: Vec<(String, String)>)
	 -> String
	{
		let oauth_header = authorization_header(
			method,
			Url::parse(api_url).unwrap(),
			None,
			&self.consumer_key,
			&self.consumer_secret,
			Some(&self.access_key),
			Some(&self.access_secret),
			HmacSha1,
			&timestamp(),
			&nonce(),
			None,
			None,
			args.into_iter()
		);

		return oauth_header;
	}
}

impl Destination<String> for TwitterClient {

	fn output(&self, data: String) -> Result<(), String>
	{
		let c = data.clone();
		return self.update_status(data).map_err(|_| c);
	}
}

unsafe impl Send for TwitterClient {}
type TwitterEmitter<'a> = Emitter<'a, String, TwitterClient>;
