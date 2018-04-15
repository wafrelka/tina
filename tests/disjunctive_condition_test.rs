extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;

fn check<C>(cond: DisjunctiveCondition<C>) -> bool
	where C: Condition
{
	let eew = EEWBuilder::new().build();
	cond.is_satisfied(&eew, None)
}

#[test]
fn it_should_return_true_if_all_clauses_are_true()
{
	let cond = vec!{TRUE_CONDITION, TRUE_CONDITION}.into();
	assert_eq!(check(cond), true);
}

#[test]
fn it_should_return_true_if_some_clauses_are_true()
{
	let cond = vec!{TRUE_CONDITION, FALSE_CONDITION}.into();
	assert_eq!(check(cond), true);
}

#[test]
fn it_should_return_false_if_all_clauses_are_false()
{
	let cond = vec!{FALSE_CONDITION, FALSE_CONDITION}.into();
	assert_eq!(check(cond), false);
}

#[test]
fn it_should_return_false_if_there_is_no_clause()
{
	let cond: DisjunctiveCondition<ConstantCondition> = vec!{}.into();
	assert_eq!(check(cond), false);
}
