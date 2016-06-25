use std::marker::Send;
use std::thread;
use std::sync::mpsc::{Sender, channel};

use eew::EEW;
use destination::Destination;


pub struct Emitter<'a, O, F>
	where O: 'static + Send, F: 'a + Fn(&EEW) -> Option<Box<O>> {
	tx: Sender<Box<O>>,
	formatter: &'a F,
}

impl<'a, O, F> Emitter<'a, O, F>
	where O: 'static + Send, F: 'a + Fn(&EEW) -> Option<Box<O>> {

	pub fn new<D>(dest: Box<D>, formatter: &'a F) -> Emitter<'a, O, F>
		where D: 'static + Destination<O> + Send
	{
		let (tx, rx) = channel::<Box<O>>();

		thread::spawn(move || {

			loop {

				let received = rx.recv().expect("data receiving should not fail");
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
			tx: tx,
			formatter: formatter,
		};

		return e;
	}

	pub fn emit(&self, eew: &EEW) -> bool
	{
		if let Some(d) = (*self.formatter)(eew) {
			self.tx.send(d).expect("data sending should not fail");
			return true;
		}
		return false;
	}
}
