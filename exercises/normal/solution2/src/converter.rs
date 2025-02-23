
pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let Some(start) = num_str.find('(') else {
        return "Invalid input".to_string();
    };

    let x: &[_] = &['(', ')'];
    let base = num_str[start..].trim_matches(x).parse::<u32>().unwrap();
    let mut iter = num_str[0..start]
        .bytes()
        .into_iter()
        .rev();

    let mut sum = 0;
    let mut idx = 0;
    while let Some(b) = iter.next() {
        sum += base.pow(idx as u32) * ((b - '0' as u8 )as u32);
        idx += 1;
    }
        
    let mut result = String::new();
    while sum > 0 {
        result.insert(0,char::from_digit(sum % to_base, to_base).unwrap());
        sum /= to_base;
    }
    result
}
