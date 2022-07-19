extern crate chrono;

use chrono::prelude::*;

#[derive(Debug)]
struct OrnellemberDate {
	day: u32,
	month: char,
	year: i32
}
impl PartialEq for OrnellemberDate {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}

fn main() {
	test();
}

fn get_month_index(date: Date<Local>) -> u32 {
	let mut month_index: u32 = date.ordinal() / 28;
	if month_index > 12 {
		month_index = 12;
	}

	month_index
}

fn get_month(month_index: u32) -> char {
	let months: [char; 13] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M'];

	months[month_index as usize]
}


fn convert_date(date: Date<Local>) -> OrnellemberDate {
	let month_index: u32 = get_month_index(date);
	let day: u32 = date.ordinal() - (month_index * 28);
	let month: char = get_month(month_index);

	OrnellemberDate { 
		day,
		month,
		year: date.year()
	}
}

fn test() {
	let cases = [
		(
			Local.ymd(2022, 01, 01),
			OrnellemberDate {day: 1, month: 'A', year: 2022}
		),
		(
			Local.yo(1969, 29),
			OrnellemberDate {day: 1, month: 'B', year: 1969}
		),
		(
			Local.yo(2022, 365),
			OrnellemberDate {day: 29, month: 'M', year: 2022}
		),
		(
			Local.yo(2020, 365),
			OrnellemberDate {day: 29, month: 'M', year: 2020}
		),
		(
			Local.yo(2020, 366),
			OrnellemberDate {day: 30, month: 'M', year: 2020}
		),
		(
			Local.ymd(2008, 08, 16),
			OrnellemberDate {day: 5, month: 'I', year: 2008}
		)
	];


	for i in 0..cases.len() {
		let input = &cases[i].0;
		let expected_output = &cases[i].1;
		assert_eq!(convert_date(*input), *expected_output);
	}

}

