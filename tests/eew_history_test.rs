extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;

fn append(his: &mut EEWHistory, eew: &EEW, ok: bool)
{
	let result = his.append(eew.clone());
	let expected = if ok { Some(eew) } else { None };

	assert_eq!(result.as_ref().map(|arc| arc.as_ref()), expected);
}

#[test]
fn it_should_hold_eews()
{
	let eew_a1 = EEWBuilder::new().id("A").number(1).build();
	let eew_a2 = EEWBuilder::new().id("A").number(2).build();
	let eew_a3 = EEWBuilder::new().id("A").number(3).build();
	let eew_b = EEWBuilder::new().id("B").number(1).build();

	let mut his = EEWHistory::new(2);

	append(&mut his, &eew_a1, true);
	append(&mut his, &eew_a2, true);
	append(&mut his, &eew_a3, true);
	append(&mut his, &eew_b, true);
}

#[test]
fn it_should_reject_non_successor_eew()
{
	let eew_1 = EEWBuilder::new().id("A").number(1).build();
	let eew_2x = EEWBuilder::new().id("A").number(2).build();
	let eew_2y = EEWBuilder::new().id("A").number(2).build();
	let eew_3 = EEWBuilder::new().id("A").number(3).build();

	let mut his = EEWHistory::new(4);

	append(&mut his, &eew_1, true);
	append(&mut his, &eew_2x, true);
	append(&mut his, &eew_2y, false);
	append(&mut his, &eew_3, true);
}

#[test]
fn it_should_erase_old_blocks_with_fifo_manner()
{
	let eew_a1 = EEWBuilder::new().id("A").number(1).build();
	let eew_b1 = EEWBuilder::new().id("B").number(1).build();
	let eew_c1 = EEWBuilder::new().id("C").number(1).build();
	let eew_c2 = EEWBuilder::new().id("C").number(2).build();
	let eew_d1 = EEWBuilder::new().id("D").number(1).build();
	let eew_d2 = EEWBuilder::new().id("D").number(2).build();

	let mut his = EEWHistory::new(3);

	append(&mut his, &eew_a1, true);
	append(&mut his, &eew_b1, true);
	append(&mut his, &eew_c1, true);
	append(&mut his, &eew_c2, true);

	append(&mut his, &eew_a1, false);
	append(&mut his, &eew_b1, false);

	append(&mut his, &eew_d1, true);
	append(&mut his, &eew_d2, true);

	append(&mut his, &eew_a1, true);
	append(&mut his, &eew_b1, true);
}
