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

fn convert_date(date: Date<Local>) -> OrnellemberDate {
	let months: [char; 13] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M'];
	
	let mut month_index: u32 = date.ordinal() / 28;
	if month_index > 12 {
		month_index = 12;
	}
	let month: char = months[month_index as usize];

	let day_of_month = date.ordinal() - (month_index * 28);

	let date: OrnellemberDate = OrnellemberDate { 
		day: day_of_month,
		month: month,
		year: date.year()
	};
	date
}

fn test() {
	let jan_1_2022: Date<Local> = Local.ymd(2022, 01, 01);
	let jan_29_1969: Date<Local> = Local.yo(1969, 29);
	let dec_31_2022: Date<Local> = Local.yo(2022, 365);
	let dec_31_2020: Date<Local> = Local.yo(2020, 366);
	let fraser_pryce_first_gold: Date<Local> = Local.ymd(2008, 08, 16);

	assert_eq!(convert_date(jan_1_2022), OrnellemberDate {day: 1, month: 'A', year: 2022});
	assert_eq!(convert_date(jan_29_1969), OrnellemberDate {day: 1, month: 'B', year: 1969});
	assert_eq!(convert_date(dec_31_2022), OrnellemberDate {day: 29, month: 'M', year: 2022});
	assert_eq!(convert_date(dec_31_2020), OrnellemberDate {day: 30, month: 'M', year: 2020});
	assert_eq!(convert_date(fraser_pryce_first_gold), OrnellemberDate {day: 5, month: 'I', year: 2008});

}

