use std::marker::{PhantomData, Send};
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};

use eew::EEW;
use destination::{Destination, TwitterClient};


unsafe impl Send for TwitterClient {}
type TwitterEmitter = Emitter<String, TwitterClient>;


pub struct Emitter<O: Send, D: Destination<O> + Send> {
	handle: thread::JoinHandle<()>,
	tx: Sender<O>,
	formatter: Box<(Fn(EEW) -> Option<O>)>,
	_marker: PhantomData<D>,
}

impl<O: 'static + Send, D: 'static + Destination<O> + Send> Emitter<O, D> {

	fn new<F>(dest: Box<D>, formatter: Box<(Fn(EEW) -> Option<O>)>) -> Emitter<O, D>
	{
		let (tx, rx) = channel();

		let t = thread::spawn(move || {

			loop {

				let r = match rx.recv() {
					Ok(d) => d,
					Err(_) => panic!()
				};

				dest.output(r);
			}
		});

		let e = Emitter {
			handle: t,
			tx: tx,
			formatter: formatter,
			_marker: PhantomData::default(),
		};

		return e;
	}
}
