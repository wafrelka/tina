use eew::{EEW, EEWPhase};
use destination::client::{SlackClient, SlackMessageType};
use destination::Destination;
use translator::ja_format_eew_short;

pub struct Slack {
	client: SlackClient,
	updown_enabled: bool,
}

impl Slack {

	pub fn build(webhook_url: &str, updown_enabled: bool) -> Result<Slack, ()>
	{
		let client = SlackClient::build(webhook_url)?;
		Ok(Slack { client: client, updown_enabled: updown_enabled })
	}
}

impl Destination for Slack {

	fn emit(&mut self, latest: &EEW, prev: Option<&EEW>)
	{
		let prev = match self.updown_enabled {
			true => prev,
			false => None,
		};

		let out = match ja_format_eew_short(latest, prev) {
			Some(out) => out,
			None => return
		};

		let body = format!("[{}] {}", out.0, out.1);
		let footer = out.2;

		let msg_type = match latest.get_eew_phase() {
			Some(EEWPhase::Alert) => SlackMessageType::Warning,
			_ => SlackMessageType::Info,
		};

		match self.client.post_message(&body, &footer, msg_type) {

			Ok(_) => {}

			Err(e) => {
				error!("SlackError: {:?}", e);
			}
		}
	}
}
