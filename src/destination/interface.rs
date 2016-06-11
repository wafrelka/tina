use std::marker::Send;

pub trait Destination<O: Send> {
	fn output(&self, data: O) -> Result<(),()>;
}
