use serde_json;
use hyper::{Url, Client, Error};
use hyper::client::Response;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::header::{Headers, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;


pub struct SlackClient {
	webhook_url: Url,
	client: Client,
}

#[derive(Debug, Clone)]
pub enum WebhookError {
	Network,
	Rejected,
}

impl SlackClient {

	pub fn build(webhook_url: &str) -> Result<SlackClient, ()>
	{
		let tls_client = NativeTlsClient::new().unwrap();
		let webhook_hyper_url = Url::parse(webhook_url).map_err(|_| ())?;

		Ok(SlackClient {
			webhook_url: webhook_hyper_url,
			client: Client::with_connector(HttpsConnector::new(tls_client)),
		})
	}

	fn post(&self, json_body: &str) -> Result<Response, Error>
	{
		let mut headers = Headers::new();
		let content_type = ContentType("application/json".parse().expect("should not fail"));
		headers.set(content_type);

		let req1 = self.client.request(Method::Post, self.webhook_url.clone())
			.headers(headers.clone()).body(json_body);

		match req1.send() {
			Ok(r) => Ok(r),
			// retry (because the first attempt may fail due to reuse of closed sockets)
			// detail: https://github.com/hyperium/hyper/issues/796
			Err(_) => {
				let req2 = self.client.request(Method::Post, self.webhook_url.clone())
					.headers(headers).body(json_body);
				req2.send()
			}
		}
	}

	pub fn post_message(&self, message: &str) -> Result<(), WebhookError>
	{
		let payload = json!({"text": message});
		let body = serde_json::to_string(&payload).expect("should not fail");
		let result = self.post(&body);
		let res = result.map_err(|_| WebhookError::Network)?;

		match res.status {

			StatusCode::Ok => Ok(()),
			_ => Err(WebhookError::Rejected),
		}
	}
}
