use crate::primes;

pub fn run() {
    let mut n = 1;
    let mut prime_tools = primes::PrimeTools::new();
    loop {
        let triangle_num = n * (n + 1) / 2;
        if prime_tools.factor_count(&triangle_num) > 500 {
            println!("{:?}", (n, triangle_num));
            break;
        }

        n += 1;
    }
}
