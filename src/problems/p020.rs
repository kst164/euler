extern crate num_bigint;
use num_bigint::BigInt;

pub fn run() {
    let mut factorial = BigInt::from(1);

    for n in 1..=100 {
        factorial *= BigInt::from(n);
    }

    println!("{}", factorial);

    let mut dig_sum = BigInt::from(0);

    while factorial > BigInt::from(0) {
        dig_sum += &factorial % 10;
        factorial /= 10;
    }

    println!("{}", dig_sum);
}
