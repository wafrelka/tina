use std::sync::Arc;

use eew::EEW;
use condition::Condition;


pub const TRUE_CONDITION: ConstantCondition = ConstantCondition(true);
pub const FALSE_CONDITION: ConstantCondition = ConstantCondition(false);

pub struct ConstantCondition(pub bool);

impl Condition for ConstantCondition {
	fn is_satisfied(&self, _: &[Arc<EEW>]) -> bool
	{
		return self.0;
	}
}
