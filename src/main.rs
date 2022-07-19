extern crate chrono;

use chrono::prelude::*;

fn main() {
	test();
}

fn is_leap_year(date: DateTime<Local>) -> bool {
	return date.year() % 4 == 0;
}

fn test() {
    let now: DateTime<Local> = Local::now();
	assert_eq!(is_leap_year(now), false);
}

