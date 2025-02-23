pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut sum = 0;
    while b < threshold {
        if b % 2 == 1 {
            sum += b;
        }

        let c = a + b;
        a = b;
        b = c;
    }
    sum
}
