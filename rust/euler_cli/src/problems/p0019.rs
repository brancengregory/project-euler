use anyhow::Result;

pub fn solve() -> Result<String> {
    let mut n_first_sundays = 0;
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut day_of_week = 2;

    for year in 1901..=2000 {
        let is_leap_year = year % 4 == 0;

        for (month_idx, &days) in month_days.iter().enumerate() {
            if day_of_week == 0 {
                n_first_sundays += 1
            };

            let n_days = if month_idx == 1 && is_leap_year {
                29
            } else {
                days
            };

            day_of_week = (day_of_week + n_days) % 7;
        }
    }

    Ok(n_first_sundays.to_string())
}

