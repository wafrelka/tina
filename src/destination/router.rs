use std::marker::Send;
use std::thread;
use std::sync::mpsc::{SyncSender, sync_channel, RecvTimeoutError};
use std::sync::Arc;
use std::time::Duration;

use eew::EEW;
use collections::IndexedLimitedQueue;
use destination::Destination;
use condition::Condition;

const CHANNEL_SIZE: usize = 256;
const EEW_BUFFER_SIZE: usize = 256;

pub struct Router<C> {
	name: String,
	tx: SyncSender<(Arc<EEW>, Option<Arc<EEW>>)>,
	cond: C,
	buffer: IndexedLimitedQueue<Arc<EEW>>,
}

pub trait Routing {
	fn emit(&mut self, eew: &Arc<EEW>);
}

impl<C> Router<C> where C: Condition {

	pub fn new<D, S>(dest: D, cond: C, name: S) -> Router<C>
		where D: Destination + Send + 'static, S: Into<String>
	{
		let (tx, rx) = sync_channel::<(Arc<EEW>, Option<Arc<EEW>>)>(CHANNEL_SIZE);

		thread::spawn(move || {

			let mut dest = dest;
			let duration = Duration::from_secs(<D as Destination>::WAKE_TIMEOUT_SECS);

			loop {
				match rx.recv_timeout(duration) {
					Ok((latest, prev)) =>
						dest.emit(&latest, prev.as_ref().map(|arc| arc.as_ref())),
					Err(RecvTimeoutError::Timeout) => dest.wake(),
					_ => panic!("eew channel closed unexpectedly")
				}
			}
		});

		let buffer = IndexedLimitedQueue::new(EEW_BUFFER_SIZE);

		Router { name: name.into(), tx: tx, cond: cond, buffer: buffer }
	}
}

impl<C> Routing for Router<C> where C: Condition {

	fn emit(&mut self, eew: &Arc<EEW>)
	{
		let name = &self.name;
		let buffer = &mut self.buffer;

		{
			let prev = buffer.get(eew.id.as_ref());

			if ! self.cond.is_satisfied(eew, prev.map(|arc| arc.as_ref())) {
				debug!("{}: eew filtered", name);
				return;
			}
		}

		let old = buffer.upsert(eew.id.as_ref(), eew.clone());

		if let Err(err) = self.tx.try_send((eew.clone(), old)) {
			warn!("Error while sending EEW data to the destination thread ({:?})", err);
		}
	}
}
