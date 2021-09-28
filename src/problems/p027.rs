use crate::primes::PrimeTools;

// Math I used: b has to be a +ve prime (n = 0)

pub fn run() {
    let mut prime_tools = PrimeTools::new();

    // Needed 2 mut references to prime_tools, one to iter and one for max_primes, and that doesn't work, so just
    // make a huge array
    let mut primes = vec![];
    for b in prime_tools.iter_primes() {
        if b > 1000000 {
            break;
        };
        primes.push(b);
    }
    println!("Filled primes");

    let mut max = 0;
    let mut max_ab = (0, 0);

    for &b in primes.iter() {
        if b > 1000 {
            break;
        }
        for a in -999..=999 {
            let m = max_primes(a, b as i64, &mut prime_tools);
            if m > max {
                max = m;
                max_ab = (a, b);
            }
        }
    }

    println!("{} {} {}", max, max_ab.0, max_ab.1);
}

fn max_primes(a: i64, b: i64, prime_tools: &mut PrimeTools) -> i64 {
    for n in 0.. {
        let f_n = f(a, b, n);
        if f_n <= 0 {
            return n;
        }
        if !(*prime_tools).is_prime(&(f_n as u64)) {
            return n;
        }
    }
    0
}

fn f(a: i64, b: i64, n: i64) -> i64 {
    n * n + a * n + b
}
