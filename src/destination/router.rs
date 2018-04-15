use std::marker::Send;
use std::thread;
use std::sync::mpsc::{SyncSender, sync_channel};
use std::sync::Arc;

use eew::EEW;
use collections::{EEWBuffer, EEWBufferError};
use destination::Destination;
use condition::Condition;

const DEFAULT_MAX_CHANNEL_SIZE: usize = 256;

pub struct Router<C> {
	name: String,
	tx: SyncSender<(Arc<EEW>, Option<Arc<EEW>>)>,
	buffer: EEWBuffer<C>,
}

pub trait Routing {
	fn emit(&mut self, eew: Arc<EEW>);
}

impl<C> Router<C> where C: Condition {

	pub fn new<D, S>(dest: D, cond: C, name: S) -> Router<C>
		where D: Destination + Send + 'static, S: Into<String>
	{
		let (tx, rx) = sync_channel::<(Arc<EEW>, Option<Arc<EEW>>)>(DEFAULT_MAX_CHANNEL_SIZE);

		thread::spawn(move || {

			let mut dest = dest;

			loop {
				let (latest, prev) = rx.recv().unwrap();
				dest.emit(&latest, prev.as_ref().map(|arc| arc.as_ref()));
			}
		});

		Router { name: name.into(), tx: tx, buffer: EEWBuffer::new(cond) }
	}
}

impl<C> Routing for Router<C> where C: Condition {

	fn emit(&mut self, eew: Arc<EEW>)
	{
		let name = &self.name;
		let buffer = &mut self.buffer;

		match buffer.append(eew) {

			Err(EEWBufferError::Order) => {
				debug!("{}: EEW Skipped (order)", name);
				return;
			},
			Err(EEWBufferError::Filter) => {
				debug!("{}: EEW Skipped (filter)", name);
				return;
			},

			Ok(list) => {

				let latest = list.latest.clone();
				let prev = list.filtered.iter().rev().nth(1).map(|e| e.clone());

				if let Err(err) = self.tx.try_send((latest, prev)) {
					warn!("Error while sending EEW data to the destination thread ({:?})", err);
				}
			}
		}
	}
}
