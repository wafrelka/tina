use std::marker::Send;
use std::thread;
use std::sync::mpsc::{SyncSender, sync_channel};
use std::sync::Arc;

use eew::EEW;
use eew_buffer::EEWBuffer;


const DEFAULT_MAX_CHANNEL_SIZE: usize = 32;

pub struct Connector {
	tx: SyncSender<Arc<EEW>>
}

impl Connector {

	pub fn new<F, A>(main_func: F, init_arg: A) -> Connector
		where F: Fn(&[Arc<EEW>], Arc<EEW>, &mut A) + Send + 'static,
			A: Send + 'static
	{
		let (tx, rx) = sync_channel::<Arc<EEW>>(DEFAULT_MAX_CHANNEL_SIZE);

		thread::spawn(move || {

			let mut buffer = EEWBuffer::new();
			let mut a = init_arg;

			loop {
				let latest = rx.recv().unwrap();
				if let Some(eews) = buffer.append(latest.clone()) {
					main_func(eews, latest, &mut a);
				}
			}
		});

		let con = Connector { tx: tx };

		return con;
	}

	pub fn emit(&self, eew: Arc<EEW>)
	{
		self.tx.try_send(eew);
	}
}
