#[cfg(test)]

extern crate chrono;
extern crate tina;

use self::chrono::*;
use self::tina::*;


#[test]
fn it_should_parse_cancel_eew()
{
	let telegram = b"39 03 10 120108133217 C11 120108133154 \
		ND20120108133201 NCN003 JD////////////// JN/// \
		/// //// ///// /// // // RK///// RT///// RC///// \
		9999=";

	let expected = EEW {
		source: Source::Tokyo,
		kind: Kind::Cancel,
		issued_at: UTC.ymd(2012, 1, 8).and_hms(4, 32, 17),
		occurred_at: UTC.ymd(2012, 1, 8).and_hms(4, 31, 54),
		id: "ND20120108133201".to_owned(),
		status: Status::Normal,
		number: 3,
		detail: EEWDetail::Cancel,
	};

	let result = parse_jma_format(telegram);

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected);
}
