use std::io::Read;

use rustc_serialize::json::Json;
use oauthcli::SignatureMethod::HmacSha1;
use oauthcli::OAuthAuthorizationHeaderBuilder;
use url::Url;
use url::form_urlencoded::Serializer;
use hyper::Url as HyperUrl;
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
		true
	}

	pub fn update_status(&self, message: &str, in_reply_to: Option<u64>)
	 -> Result<u64, StatusUpdateError>
	{
		let api_url = "https://api.twitter.com/1.1/statuses/update.json";
		let prev_str_opt = in_reply_to.map(|i| i.to_string());
		let mut args = vec![("status", message)];
		if let Some(prev_str) = prev_str_opt.as_ref() {
			args.push(("in_reply_to_status_id", prev_str));
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
		let mut body = String::new();
		let mut headers = Headers::new();
		let mut url_obj = HyperUrl::parse(api_url).unwrap();

		if method == Method::Post || method == Method::Get {

			let mut args_serializer = Serializer::new(String::new());
			for &(k, v) in args.iter() {
				args_serializer.append_pair(k, v);
			}
			let args_serialized = args_serializer.finish();

			if method == Method::Post {
				body.push_str(&args_serialized);
				let content_type = ContentType("application/x-www-form-urlencoded".parse().unwrap());
				headers.set(content_type);
			} else {
				url_obj.set_query(Some(&args_serialized));
			}
		}

		let oauth_header = self.construct_oauth_header(&method, api_url, args);
		headers.set(Authorization(oauth_header));

		let m = method.clone();
		let h = headers.clone();
		let u = url_obj.clone();
		if let Ok(r) = self.client.request(m, u).headers(h).body(&body).send() {
			return Ok(r);
		}

		// retry (because the first attempt may fail due to reuse of closed sockets)
		// detail: https://github.com/hyperium/hyper/issues/796
		return self.client.request(method, url_obj).headers(headers).body(&body).send();
	}

	fn construct_oauth_header(&self, method: &Method, api_url: &str, args: Vec<(&str, &str)>)
	 -> String
	{
		OAuthAuthorizationHeaderBuilder::new(
			method.as_ref(),
			&Url::parse(api_url).expect("must be a valid url string"),
			self.consumer_key.as_ref(),
			self.consumer_secret.as_ref(),
			HmacSha1)
			.token(self.access_key.as_ref(), self.access_secret.as_ref())
			.request_parameters(args.into_iter())
			.finish_for_twitter()
			.to_string()
	}
}
