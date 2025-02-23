// N=p+2pow(k,2)
pub fn goldbach_conjecture() -> String {
    let mut prime_vec = Vec::new(); // 质数集
    let mut odd_vec = Vec::new(); // 搜索集
    let mut even_vec = Vec::new(); // 2pow(k,2)

    for i in 1..=10000 {
        if is_prime(i) {
            prime_vec.push(i);
        } else if i % 2 == 1 && i != 1 {
            odd_vec.push(i);
        }

        even_vec.push(i.pow(2) * 2);
    }

    let mut result = Vec::new();
    'outer: for i in 0..=odd_vec.len() {
        let mut flag = false;
        for j in 0..=even_vec.len() {
            if even_vec[j] >= odd_vec[i] {
                break;
            }

            let diff = odd_vec[i] - even_vec[j];
            if prime_vec.binary_search(&diff).is_ok() {
                flag = true;
                break;
            }
        }
        if !flag {
            result.push(odd_vec[i].to_string());

            if result.len() == 2 {
                break 'outer;
            }
        }
    }

    result.join(",").to_string()
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
