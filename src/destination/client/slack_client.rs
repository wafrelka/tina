use reqwest::{Client, Url, StatusCode};

const WARNING_COLOR: &'static str = "#CF0301";
const INFO_COLOR: &'static str = "#E8E8E8";

pub struct SlackClient {
	webhook_url: Url,
	client: Client,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SlackError {
	Network,
	Rejected,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SlackMessageType {
	Warning,
	Info,
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

	pub fn post_message(&self, body: &str, footer: &str,
		msg_type: SlackMessageType) -> Result<(), SlackError>
	{
		let color = match msg_type {
			SlackMessageType::Warning => WARNING_COLOR,
			SlackMessageType::Info => INFO_COLOR,
		};

		let payload = json!({
			"attachments" : [{
				"fallback" : body,
				"text" : body,
				"footer": footer,
				"color" : color,
			}]
		});

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
