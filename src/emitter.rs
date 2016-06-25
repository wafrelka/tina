use std::marker::Send;
use std::thread;
use std::sync::mpsc::{Sender, channel};

use eew::EEW;
use destination::Destination;


pub struct Emitter<'a, O, F>
	where O: 'static + Send, F: 'a + Fn(&EEW) -> Option<Box<O>> {
	handle: thread::JoinHandle<()>,
	tx: Sender<Box<O>>,
	formatter: &'a F,
}

impl<'a, O, F> Emitter<'a, O, F>
	where O: 'static + Send, F: 'a + Fn(&EEW) -> Option<Box<O>> {

	pub fn new<D>(dest: Box<D>, formatter: &'a F) -> Emitter<'a, O, F>
		where D: 'static + Destination<O> + Send
	{
		let (tx, rx) = channel::<Box<O>>();

		let handle = thread::spawn(move || {

			loop {

				let received = match rx.recv() {
					Ok(data) => data,
					Err(_) => panic!()
				};

				let mut formatted = *received;

				loop {

					if let Err(returned) = dest.output(formatted) {
						formatted = returned;
					} else {
						break;
					}
				}
			}
		});

		let e = Emitter {
			handle: handle,
			tx: tx,
			formatter: formatter,
		};

		return e;
	}

	pub fn emit(&self, eew: &EEW) -> bool
	{
		if let Some(d) = (*self.formatter)(eew) {
			match self.tx.send(d) {
				Ok(_) => return true,
				_ => panic!()
			};
		}
		return false;
	}
}
