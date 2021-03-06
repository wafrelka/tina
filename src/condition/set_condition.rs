use eew::EEW;
use condition::Condition;

pub struct DisjunctiveCondition<C> {
	clauses: Vec<C>,
}

impl<C> DisjunctiveCondition<C> {

	fn new(clauses: Vec<C>) -> DisjunctiveCondition<C>
	{
		DisjunctiveCondition { clauses: clauses }
	}
}

impl<C> Condition for DisjunctiveCondition<C> where C: Condition {

	fn is_satisfied(&self, latest: &EEW, prev: Option<&EEW>) -> bool
	{
		self.clauses.iter().any(|c| c.is_satisfied(latest, prev))
	}
}

impl<C> From<Vec<C>> for DisjunctiveCondition<C> {

	fn from(clauses: Vec<C>) -> DisjunctiveCondition<C>
	{
		DisjunctiveCondition::new(clauses)
	}
}
