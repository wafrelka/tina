use std::marker::Send;
use std::thread;
use std::sync::mpsc::{SyncSender, sync_channel};
use std::sync::Arc;

use eew::EEW;
use collections::{EEWBuffer, EEWBufferError};
use destination::Destination;
use condition::Condition;


const DEFAULT_MAX_CHANNEL_SIZE: usize = 32;

pub struct EEWSocket {
	tx: SyncSender<Arc<EEW>>
}

impl EEWSocket {

	pub fn new<D, C>(dest: D, cond: C) -> EEWSocket
		where D: Destination + Send + 'static, C: Condition + Send + 'static
	{
		let (tx, rx) = sync_channel::<Arc<EEW>>(DEFAULT_MAX_CHANNEL_SIZE);

		thread::spawn(move || {

			let mut buffer = EEWBuffer::new(cond);
			let mut dest = dest;

			loop {

				let latest = rx.recv().unwrap();

				match buffer.append(latest) {
					Ok(list) => { dest.emit(&list.latest, &list.filtered); },
					Err(EEWBufferError::Order) => { info!("EEW Skipped (order)") },
					Err(EEWBufferError::Filter) => { info!("EEW Skipped (filter)"); },
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
