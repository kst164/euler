extern crate num_bigint;
use num_bigint::BigInt;

pub fn run() {
    let mut num = BigInt::from(2);
    num = num.pow(15);
    let mut dig_sum = BigInt::from(0);

    while &num > &BigInt::from(0) {
        dig_sum += &num % 10;
        num /= 10;
    }

    println!("{}", dig_sum);
}
