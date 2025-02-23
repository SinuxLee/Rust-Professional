pub fn find_max_prime_factor(mut number: u128) -> u128 {
    if number == 0 {
        return 0;
    }
    let mut max_prime = 1;

    // 处理2的因子
    if number % 2 == 0 {
        max_prime = 2;
        while number % 2 == 0 {
            number /= 2;
        }
    }

    // 处理其他小因子，例如到1000
    let mut i = 3;
    while i <= 1000 && i <= number {
        if number % i == 0 {
            max_prime = max_prime.max(i);
            while number % i == 0 {
                number /= i;
            }
        }
        i += 2;
    }

    if number == 1 {
        return max_prime;
    }

    // 分解剩余的部分
    let factors = factorize(number);
    let max_factor = factors.iter().max().copied().unwrap_or(1);

    max_prime.max(max_factor)
}

fn factorize(n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    if n == 1 {
        return factors;
    }
    if is_prime(n) {
        factors.push(n);
        return factors;
    }
    let d = pollards_rho(n);
    let mut factors_d = factorize(d);
    factors.append(&mut factors_d);
    let mut factors_nd = factorize(n / d);
    factors.append(&mut factors_nd);
    factors
}

fn pollards_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }

    let mut c = 1;
    loop {
        let f = |x: u128| -> u128 { (pow_mod(x, 2, n) + c) % n };
        let mut x = 2;
        let mut y = 2;
        let mut d = 1;

        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd(x.abs_diff(y), n);
        }

        if d != n {
            return d;
        }

        c += 1;
        if c > 100 {
            return n;
        }
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in bases.iter() {
        if a >= n {
            continue;
        }
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 0..s - 1 {
            x = pow_mod(x, 2, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

fn pow_mod(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut result = 1;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            result = (result * a) % m;
        }
        a = (a * a) % m;
        b /= 2;
    }
    result
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}