use std::io::Read;

use rustc_serialize::json::Json;
use oauthcli::SignatureMethod::HmacSha1;
use oauthcli::OAuthAuthorizationHeaderBuilder;
use url::Url;
use url::form_urlencoded::Serializer;
use hyper::{Client, Error};
use hyper::client::Response;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::header::{Headers, Authorization, ContentType};


pub struct TwitterClient {
	consumer_key: String,
	consumer_secret: String,
	access_key: String,
	access_secret: String,
	client: Client
}

#[derive(Debug, Clone)]
pub enum StatusUpdateError {
	Duplicated,
	RateLimitExceeded,
	InvalidTweet,
	InvalidResponse,
	Network,
	Unauthorized,
	Unknown(String)
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

	pub fn update_status(&self, message: &str, in_reply_to: Option<u64>)
	 -> Result<u64, StatusUpdateError>
	{
		let api_url = "https://api.twitter.com/1.1/statuses/update.json";
		let prev_str_opt = in_reply_to.map(|i| i.to_string());
		let mut args = vec![("status", message)];
		if prev_str_opt.is_some() {
			args.push(("in_reply_to_status_id", prev_str_opt.as_ref().unwrap()));
		}
		let result = self.request(Method::Post, api_url, args);

		let mut res = try!(result.map_err(|_| StatusUpdateError::Network));

		match res.status {

			StatusCode::Forbidden => {

				let mut body = String::new();

				try!(res.read_to_string(&mut body).map_err(|_| StatusUpdateError::Network));

				if body.contains("140") {
					return Err(StatusUpdateError::InvalidTweet);
				} else if body.contains("duplicate") {
					return Err(StatusUpdateError::Duplicated);
				}

				return Err(StatusUpdateError::Unknown(body));
			},

			StatusCode::Ok => {

				let mut body = String::new();

				try!(res.read_to_string(&mut body).map_err(|_| StatusUpdateError::Network));

				let json = try!(Json::from_str(&body).map_err(|_| StatusUpdateError::InvalidResponse));
				let id_obj = try!(json.find("id").ok_or(StatusUpdateError::InvalidResponse));
				let id = try!(id_obj.as_u64().ok_or(StatusUpdateError::InvalidResponse));

				return Ok(id);
			},

			StatusCode::TooManyRequests => return Err(StatusUpdateError::RateLimitExceeded),
			StatusCode::Unauthorized => return Err(StatusUpdateError::Unauthorized),
			_ => return Err(StatusUpdateError::Unknown(format!("Unknown status: {}", res.status)))
		};
	}

	fn request(&self, method: Method, api_url: &str, args: Vec<(&str, &str)>)
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

	fn construct_oauth_header(&self, method: &str, api_url: &str, args: Vec<(&str, &str)>)
	 -> String
	{
		let oauth_header = OAuthAuthorizationHeaderBuilder::new(
			method,
			&Url::parse(api_url).unwrap(),
			self.consumer_key.as_ref(),
			self.consumer_secret.as_ref(),
			HmacSha1)
			.token(self.access_key.as_ref(), self.access_secret.as_ref())
			.request_parameters(args.into_iter())
			.finish_for_twitter();

		return oauth_header.to_string();
	}
}
