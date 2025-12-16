use anyhow::Result;

pub fn solve() -> Result<String> {
    let mut n_first_sundays = 0;
    let month_days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut day_of_week = 0;

    for year in 1901..=2000 {
        let is_leap_year = year % 4 == 0;

        month_days.iter().enumerate().for_each(|(month, days)| {
            let n_days = if is_leap_year && month == 1 {
                days + 1
            } else {
                *days
            };

            for day in 0..n_days {
                day_of_week = (day_of_week + 1) % 7;
                if day == 0 && day_of_week == 6 {
                    n_first_sundays += 1;
                }
            }
        });
    }

    Ok(n_first_sundays.to_string())
}

