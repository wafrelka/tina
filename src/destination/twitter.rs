use eew::EEW;
use destination::client::TwitterClient;
use destination::Destination;
use collections::IndexedLimitedQueue;
use translator::ja_format_eew_short;

pub struct Twitter {
	client: TwitterClient,
	latest_tw_ids: IndexedLimitedQueue<u64>,
	reply_enabled: bool,
	updown_enabled: bool,
}

impl Twitter {

	pub fn new(consumer_key: String, consumer_secret: String,
		access_key: String, access_secret: String, reply_enabled: bool, updown_enabled: bool) -> Twitter
	{
		let client = TwitterClient::new(consumer_key, consumer_secret, access_key, access_secret);
		let q = IndexedLimitedQueue::new(16);

		Twitter { client: client, latest_tw_ids: q,
			reply_enabled: reply_enabled, updown_enabled: updown_enabled }
	}

	pub fn is_valid(&self) -> bool
	{
		self.client.is_valid()
	}
}

impl Destination for Twitter {

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

		let prev_tw_id = match self.reply_enabled {
			true => self.latest_tw_ids.get(latest.id.as_ref()).map(|id| *id),
			false => None
		};

		match self.client.update_status(&out, prev_tw_id) {

			Ok(tw_id) => {
				self.latest_tw_ids.upsert(latest.id.clone(), tw_id);
			},

			Err(e) => {
				error!("TwitterError: {:?}", e);
			}
		}
	}
}
