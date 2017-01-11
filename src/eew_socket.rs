use std::marker::Send;
use std::thread;
use std::sync::mpsc::{SyncSender, sync_channel};
use std::sync::Arc;

use eew::EEW;
use collections::EEWBuffer;
use destination::Destination;


const DEFAULT_MAX_CHANNEL_SIZE: usize = 32;

pub struct EEWSocket {
	tx: SyncSender<Arc<EEW>>
}

impl EEWSocket {

	pub fn new<D>(dest: D) -> EEWSocket where D: Destination + Send + 'static
	{
		let (tx, rx) = sync_channel::<Arc<EEW>>(DEFAULT_MAX_CHANNEL_SIZE);

		thread::spawn(move || {

			let mut buffer = EEWBuffer::new();
			let mut dest = dest;

			loop {
				let latest = rx.recv().unwrap();
				if let Some(eews) = buffer.append(latest.clone()) {
					dest.emit(eews, latest);
				}
			}
		});

		let sock = EEWSocket { tx: tx };

		sock
	}

	pub fn emit(&self, eew: Arc<EEW>)
	{
		if let Err(err) = self.tx.try_send(eew) {
			warn!("Error while sending EEW data to the destination thread ({:?})", err);
		}
	}
}
