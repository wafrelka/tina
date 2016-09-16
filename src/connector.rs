use std::marker::Send;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::sync::Arc;

use eew::EEW;
use eew_buffer::EEWBuffer;


pub struct Connector {
	tx: Sender<Arc<EEW>>
}

impl Connector {

	pub fn new<F, A>(main_func: F, init_arg: A) -> Connector
		where F: Fn(&[EEW], &EEW, &mut A) + Send + 'static,
			A: Send + 'static
	{
		let (tx, rx) = channel::<Arc<EEW>>();

		thread::spawn(move || {

			let mut buffer = EEWBuffer::new();
			let mut a = init_arg;

			loop {
				let latest = rx.recv().unwrap();
				if let Some(ref eews) = buffer.append(&latest) {
					main_func(&eews, &latest, &mut a);
				}
			}
		});

		let con = Connector { tx: tx };

		return con;
	}

	pub fn emit(&self, eew: Arc<EEW>)
	{
		self.tx.send(eew);
	}
}
