use std::io::Read;

use oauthcli::SignatureMethod::HmacSha1;
use oauthcli::OAuthAuthorizationHeaderBuilder;
use serde_json;
use serde_json::Value;
use reqwest::{Client, Url, StatusCode, Response};
use reqwest::header::{Authorization};

const API_URL: &'static str = "https://api.twitter.com/1.1/statuses/update.json";

pub struct TwitterClient {
	consumer_key: String,
	consumer_secret: String,
	access_key: String,
	access_secret: String,
	client: Client,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TwitterError {
	Duplicated,
	RateLimitExceeded,
	InvalidTweet,
	InvalidResponse,
	Network,
	Unauthorized,
	Unknown(String),
}

impl TwitterClient {

	pub fn new(consumer_key: String, consumer_secret: String, access_key: String, access_secret: String) -> TwitterClient
	{
		TwitterClient {
			consumer_key: consumer_key,
			consumer_secret: consumer_secret,
			access_key: access_key,
			access_secret: access_secret,
			client: Client::new(),
		}
	}

	pub fn is_valid(&self) -> bool
	{
		// TODO: implement token validation
		true
	}

	fn post(&self, url: &str, args: Vec<(&str, &str)>) -> Result<Response, ()>
	{
		let mut req = self.client.post(url);

		req.form(&args);

		let oauth_header = OAuthAuthorizationHeaderBuilder::new(
			"POST",
			&Url::parse(url).expect("must be a valid url string"),
			&self.consumer_key,
			&self.consumer_secret,
			HmacSha1)
			.token(&self.access_key, &self.access_secret)
			.request_parameters(args.into_iter())
			.finish_for_twitter()
			.to_string();

		req.header(Authorization(oauth_header)).send().map_err(|_| ())
	}

	pub fn update_status(&self, message: &str, in_reply_to: Option<u64>)
	 -> Result<u64, TwitterError>
	{
		let prev = in_reply_to.map(|i| i.to_string());
		let mut args = vec![("status", message)];

		if let Some(prev) = prev.as_ref() {
			args.push(("in_reply_to_status_id", prev));
		}

		let mut response = self.post(API_URL, args).map_err(|_| TwitterError::Network)?;

		match response.status() {

			StatusCode::Forbidden => {

				let mut body = String::new();
				response.read_to_string(&mut body).map_err(|_| TwitterError::Network)?;

				let json: Option<Value> = serde_json::from_str(&body).ok();
				let id = json.and_then(|json| json["errors"][0]["code"].as_i64());

				match id {
					Some(186) => Err(TwitterError::InvalidTweet),
					Some(187) => Err(TwitterError::Duplicated),
					_ => Err(TwitterError::Unknown(body))
				}
			},

			StatusCode::Ok => {

				let mut body = String::new();
				response.read_to_string(&mut body).map_err(|_| TwitterError::Network)?;

				let json: Value =
					serde_json::from_str(&body).map_err(|_| TwitterError::InvalidResponse)?;
				let id = json["id"].as_u64().ok_or(TwitterError::InvalidResponse)?;

				Ok(id)
			},

			StatusCode::TooManyRequests => Err(TwitterError::RateLimitExceeded),
			StatusCode::Unauthorized => Err(TwitterError::Unauthorized),
			_ => Err(TwitterError::Unknown(format!("unknown status: {}", response.status())))
		}
	}
}
