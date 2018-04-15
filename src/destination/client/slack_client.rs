use reqwest::{Client, Url, StatusCode};

pub struct SlackClient {
	webhook_url: Url,
	client: Client,
}

#[derive(Debug, Clone)]
pub enum SlackError {
	Network,
	Rejected,
}

impl SlackClient {

	pub fn build(webhook_url: &str) -> Result<SlackClient, ()>
	{
		let webhook_url = Url::parse(webhook_url).map_err(|_| ())?;

		Ok(SlackClient {
			webhook_url: webhook_url,
			client: Client::new(),
		})
	}

	pub fn post_message(&self, message: &str) -> Result<(), SlackError>
	{
		let payload = json!({"text": message});

		let response = self.client.post(self.webhook_url.clone())
			.json(&payload)
			.send()
			.map_err(|_| SlackError::Network)?;

		match response.status() {
			StatusCode::Ok => Ok(()),
			_ => Err(SlackError::Rejected),
		}
	}
}
