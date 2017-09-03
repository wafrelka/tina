#![cfg(test)]

extern crate chrono;
extern crate tina;

use std::sync::Arc;

use chrono::*;
use tina::*;


fn check<C>(cond: DisjunctiveCondition<C>) -> bool
	where C: Condition
{
	let eew = Arc::new(EEW {
		issue_pattern: IssuePattern::Cancel, source: Source::Tokyo, kind: Kind::Cancel,
		issued_at: UTC.timestamp(12345, 0), occurred_at: UTC.timestamp(12345, 0),
		id: "XXX".to_string(), status: Status::Normal, number: 1, detail: None
	});
	cond.is_satisfied(&eew.clone(), vec!{eew}.as_slice())
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
