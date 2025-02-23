pub fn dp_rec_mc(amount: u32) -> u32 {
    const CASHES: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];
    let mut count = 0;
    let mut total = amount;
    let mut idx = 0;
    while total > 0 {
        if total >= CASHES[idx] {
            total -= CASHES[idx];
            count += 1;
        } else {
            idx += 1;
        }
    }
    count
}
