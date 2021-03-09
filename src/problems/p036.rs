pub fn run() {
    let mut sum = 0;

    for n in (1..1000000).step_by(2) {
        if is_base_10_palindrome(n) && is_base_2_palindrome(n) {
            sum += n;
        }
    }

    println!("{}", sum);
}

fn is_base_10_palindrome(mut n: i32) -> bool {
    // Digit reversed form of n;
    let mut rev = 0;

    let original_num = n;

    while n > 0 {
        let last_digit = n % 10;
        n /= 10;

        rev = 10 * rev + last_digit;
    }

    rev == original_num
}

fn is_base_2_palindrome(mut n: i32) -> bool {
    let mut rev = 0;

    let original_num = n;

    while n > 0 {
        let last_digit = n & 1;
        n >>= 1;

        rev = (rev << 1) + last_digit;
    }

    rev == original_num
}
