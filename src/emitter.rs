use std::marker::{PhantomData, Send};
use std::thread;
use std::sync::mpsc::{Sender, channel};

use eew::EEW;
use destination::Destination;


pub struct Emitter<'a, O, D>
	where O: 'static + Send, D: 'static + Destination<O> + Send {
	handle: thread::JoinHandle<()>,
	tx: Sender<Box<O>>,
	formatter: &'a (Fn(&EEW) -> Option<Box<O>>),
	_marker: PhantomData<D>,
}

impl<'a, O, D> Emitter<'a, O, D>
	where O: 'static + Send, D: 'static + Destination<O> + Send {

	pub fn new(dest: Box<D>, formatter: &'a Fn(&EEW) -> Option<Box<O>>) -> Emitter<'a, O, D>
	{
		let (tx, rx) = channel::<Box<O>>();

		let handle = thread::spawn(move || {

			loop {

				let formatted_box = match rx.recv() {
					Ok(data) => data,
					Err(_) => panic!()
				};

				let mut formatted = *formatted_box;

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
			_marker: PhantomData::default(),
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
