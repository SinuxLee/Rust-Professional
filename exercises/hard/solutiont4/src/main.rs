//Calculated according to ISO8061 standard

mod calc_time;

fn main() {
    let times = vec![
        "2025-01-01",
        "2025-01-18",

        "2025-12-31",
        "2025-11-01",

        "2025-02-28",
        "2025-04-01",

        "2025-01-28",
        "2025-01-30",

        "2025-02-09",
        "2025-05-01",
    ];
    for time in times {
        let result = calc_time::time_info(time);
        println!("{}", result);
    }
}
