#[derive(PartialEq, Eq, Debug)]
enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}
impl Month {
    fn next(&mut self) -> Self {
        match self {
            Month::Jan => Month::Feb,
            Month::Feb => Month::Mar,
            Month::Mar => Month::Apr,
            Month::Apr => Month::May,
            Month::May => Month::Jun,
            Month::Jun => Month::Jul,
            Month::Jul => Month::Aug,
            Month::Aug => Month::Sep,
            Month::Sep => Month::Oct,
            Month::Oct => Month::Nov,
            Month::Nov => Month::Dec,
            Month::Dec => Month::Jan,
        }
    }
    fn get_number_of_days(&self, is_leap_year: bool) -> u8 {
        match self {
            Month::Feb => {
                if is_leap_year {
                    29
                } else {
                    28
                }
            }
            Month::Apr | Month::Jun | Month::Sep | Month::Nov => 30,
            Month::Jan
            | Month::Mar
            | Month::May
            | Month::Jul
            | Month::Aug
            | Month::Oct
            | Month::Dec => 31,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}
impl Day {
    fn next(&self) -> Self {
        match self {
            Day::Mon => Day::Tue,
            Day::Tue => Day::Wed,
            Day::Wed => Day::Thu,
            Day::Thu => Day::Fri,
            Day::Fri => Day::Sat,
            Day::Sat => Day::Sun,
            Day::Sun => Day::Mon,
        }
    }
}

pub fn counting_sundays() -> usize {
    let (mut date, mut day, mut month, mut year) = (1, Day::Tue, Month::Jan, 1901);

    let mut num_of_1sundays = 0;
    while date != 2 || month != Month::Dec || year != 2000 {
        // println!("The {date} {month:?} {year} was a {day:?}");
        let is_leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

        date += 1;
        day = day.next();

        if date > month.get_number_of_days(is_leap_year) {
            date = 1;
            month = month.next();
            if month == Month::Jan {
                year += 1;
            }

            if day == Day::Sun {
                num_of_1sundays += 1;
            }
        }
    }

    num_of_1sundays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_19_tests() {
        assert_eq!(counting_sundays(), 171);
    }
}
