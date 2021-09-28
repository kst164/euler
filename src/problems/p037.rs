use crate::primes::PrimeTools;

pub fn run() {
    let mut iter_pt = PrimeTools::new();
    let mut pt = PrimeTools::new();
    let mut sum = 0;
    let mut count = 0;

    for p in iter_pt.iter_primes() {
        if p < 10 {
            continue;
        }
        if is_truncatable(p, &mut pt) {
            println!("{}", p);
            sum += p;
            count += 1;

            if count == 11 {
                break;
            }
        }
    }

    println!("sum: {}", sum);
}

fn is_truncatable(p: u64, pt: &mut PrimeTools<u64>) -> bool {
    let mut prime = p;
    let mut reverse = 0;
    let mut dig_count = 0;

    // Check R to L truncatability while building reverse of digits of p
    //println!("Part 1");
    while prime > 0 {
        //println!("{}", prime);
        if !pt.is_prime(&prime) {
            return false;
        }

        reverse = 10 * reverse + prime % 10;
        prime /= 10;
        dig_count += 1;
    }

    prime = p;

    // Check L to R truncatability by truncating using reverse and dig_count
    //println!("Part 2");
    while prime > 0 {
        //println!("{}", prime);
        if !pt.is_prime(&prime) {
            return false;
        }

        prime -= (reverse % 10) * 10u64.pow(dig_count - 1);
        reverse /= 10;
        dig_count -= 1;
    }

    true
}
