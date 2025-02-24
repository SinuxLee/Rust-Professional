static DAY_OF_WEEK: i32 = 3;
static WEEK_DAY: [i32; 7] = [7, 1, 2, 3, 4, 5, 6]; // 周日是每周第一天

pub fn time_info(time: &str) -> String {
    let time = time.split("-").collect::<Vec<&str>>();
    let year = time[0].parse::<i32>().unwrap();
    let month = time[1].parse::<i32>().unwrap();
    let day = time[2].parse::<i32>().unwrap();

    let days = days_since_new_year(year, month, day);
    let week_of_year = ((DAY_OF_WEEK + days + 5) / 7) % 52;
    let day_of_week = WEEK_DAY[((DAY_OF_WEEK + days - 1) % 7) as usize];
    let remaining_days_of_year = 365 - days;
    let days_since_spring_festival = days_since_spring_festival(year, month, day);

    let mut idx = 0;

    let mut current = next_day(year, month, day);
    while is_holiday(current.0, current.1, current.2) {
        current = next_day(current.0, current.1, current.2);
        idx += 1;
    }

    format!(
        "{},{},{},{},{},{}",
        week_of_year, day_of_week, days, remaining_days_of_year, days_since_spring_festival, idx
    )
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn days_since_new_year(year: i32, month: i32, day: i32) -> i32 {
    let mut days = day;
    for m in 1..month {
        days += days_in_month(year, m);
    }
    days
}

fn days_since_spring_festival(year: i32, month: i32, day: i32) -> i32 {
    static SPRING_FESTIVAL_2025: (i32, i32, i32) = (2025, 1, 29);
    static SPRING_FESTIVAL_2026: (i32, i32, i32) = (2026, 2, 17);

    if month == SPRING_FESTIVAL_2025.1 && day <= SPRING_FESTIVAL_2025.2 {
        return SPRING_FESTIVAL_2025.2 - day;
    }

    let mut days = 0;

    for m in 1..SPRING_FESTIVAL_2026.1 {
        days += days_in_month(year, m);
    }
    days += SPRING_FESTIVAL_2026.2 - 1;

    for m in (month + 1)..13 {
        days += days_in_month(year, m);
    }

    days += days_in_month(year, month) - day + 1;

    days
}

fn next_day(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    if day == days_in_month(year, month) {
        (year, (month + 1) % 12, 1)
    } else {
        (year, month, day + 1)
    }
}

// https://english.www.gov.cn/policies/latestreleases/202411/12/content_WS67331db5c6d0868f4e8ecd92.html
fn is_holiday(year: i32, month: i32, day: i32) -> bool {
    static HOLIDAYS: &[(i32, i32, i32)] = &[
        (2025, 1, 1),  // Jan 1
        (2025, 1, 28), // Jan 28 — Feb 4
        (2025, 1, 29),
        (2025, 1, 30),
        (2025, 1, 31),
        (2025, 2, 1),
        (2025, 2, 2),
        (2025, 2, 3),
        (2025, 2, 4),
        (2025, 4, 4), // April 4 — April 6
        (2025, 4, 5),
        (2025, 4, 6),
        (2025, 5, 1), // May 1 — May 5
        (2025, 5, 2),
        (2025, 5, 3),
        (2025, 5, 4),
        (2025, 5, 5),
        (2025, 5, 31), // May 31 — June 2
        (2025, 6, 1),
        (2025, 6, 2),
        (2025, 10, 1), // Oct 1 — Oct 8
        (2025, 10, 2),
        (2025, 10, 3),
        (2025, 10, 4),
        (2025, 10, 5),
        (2025, 10, 6),
        (2025, 10, 7),
        (2025, 10, 8),
    ];

    // 法定节假日
    if HOLIDAYS
        .iter()
        .any(|h| h.0 == year && h.1 == month && h.2 == day)
    {
        return true;
    }

    // 周末
    let day_of_week = (DAY_OF_WEEK + days_since_new_year(year, month, day) - 1) % 7;
    if day_of_week == 0 || day_of_week == 6 {
        return true;
    }

    false
}
