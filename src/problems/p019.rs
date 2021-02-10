pub fn run() {
    //let days_in_month: [u8] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let days_in_month: [u8; 12] = [3, 0, 3, 2, 3, 2, 3, 3, 2, 3, 2, 3];

    // Sunday is 0, Monday is 1, ...
    // Starting on 1 Jan 1900
    let mut day_of_week: u8 = 1;
    let mut month: usize = 0;
    let mut year: u16 = 1900;

    let mut sundays_on_1st: usize = 0;

    while year <= 2000 {
        if year > 1900 && day_of_week == 0 {
            sundays_on_1st += 1;
        }

        day_of_week += days_in_month[month];
        if month == 1 && is_leap_year(&year) {
            day_of_week += 1;
        }
        day_of_week %= 7;

        if month == 11 {
            month = 0;
            year += 1;
        } else {
            month += 1;
        }
    }

    println!("{}", sundays_on_1st);
}

fn is_leap_year(year: &u16) -> bool {
    if year % 4 != 0 { return false; }
    if year % 400 == 0 { return true; }
    return year % 100 != 0;
}
