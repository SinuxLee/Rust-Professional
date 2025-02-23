pub fn new_count_distinct(input_str: &str) -> usize {
    let parts = input_str.split(",").collect::<Vec<&str>>();
    let mut distinct_parts = Vec::new();
    for part in parts {
        if !distinct_parts.contains(&part) {
            distinct_parts.push(part);
        }
    }
    distinct_parts.len()
}
