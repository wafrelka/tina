use std::sync::Arc;

use eew::EEW;
use destination::client::TwitterClient;
use destination::Destination;
use collections::LimitedQueue;
use translator::ja_format_eew_short;


pub struct Twitter {
	client: TwitterClient,
	latest_tw_ids: LimitedQueue<(String, u64)>,
	reply_enabled: bool
}

impl Twitter {

	pub fn new(consumer_key: String, consumer_secret: String,
		access_key: String, access_secret: String, reply_enabled: bool) -> Twitter
	{
		let client = TwitterClient::new(consumer_key, consumer_secret, access_key, access_secret);
		let q = LimitedQueue::new(16);

		let tw = Twitter { client: client, latest_tw_ids: q, reply_enabled: reply_enabled };

		tw
	}

	pub fn is_valid(&self) -> bool
	{
		self.client.is_valid()
	}
}

impl Destination for Twitter {

	fn emit(&mut self, eews: &[Arc<EEW>], latest: Arc<EEW>)
	{
		let out = match ja_format_eew_short(&latest, eews.iter().rev().nth(1).map(|e| e.as_ref())) {
			Some(out) => out,
			None => return
		};

		let prev_tw_id_opt = match self.reply_enabled {
			true => self.latest_tw_ids.iter().find(|x| x.0 == latest.id).map(|x| x.1),
			false => None
		};

		match self.client.update_status(&out, prev_tw_id_opt) {

			Ok(tw_id) => {

				if prev_tw_id_opt == None {
					self.latest_tw_ids.push((latest.id.clone(), tw_id));
				} else {
					self.latest_tw_ids.iter_mut().find(|x| x.0 == latest.id).unwrap().1 = tw_id;
				}
			},

			Err(e) => {
				error!("TwitterError: {:?}", e);
			}
		}
	}
}
