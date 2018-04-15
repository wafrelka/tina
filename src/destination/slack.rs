use eew::EEW;
use destination::client::SlackClient;
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

		match self.client.post_message(&out) {

			Ok(_) => {}

			Err(e) => {
				error!("SlackError: {:?}", e);
			}
		}
	}
}
