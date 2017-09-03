use std::sync::Arc;

use eew::EEW;
use destination::client::SlackClient;
use destination::Destination;
use translator::ja_format_eew_short;


pub struct Slack {
	client: SlackClient,
}

impl Slack {

	pub fn build(webhook_url: &str) -> Result<Slack, ()>
	{
		let client = SlackClient::build(webhook_url)?;
		Ok(Slack { client: client })
	}
}

impl Destination for Slack {

	fn emit(&mut self, latest: &Arc<EEW>, eews: &[Arc<EEW>])
	{
		let out = match ja_format_eew_short(&latest, eews.iter().rev().nth(1).map(|e| e.as_ref())) {
			Some(out) => out,
			None => return
		};

		match self.client.post_message(&out) {

			Ok(_) => {}

			Err(e) => {
				error!("SlackError: {:?}", e);
			}
		}
	}
}
