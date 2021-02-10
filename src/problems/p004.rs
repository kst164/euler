pub fn run() {
    let mut max_palindrome = 1;

    for a in (1..1000).rev() {
        for b in (1..a).rev() {
            if a * b > max_palindrome {
                if is_2k_digit_palindrome(&(a * b), 3) {
                    max_palindrome = a * b;
                    println!("{:3} * {:3} = {:6}", a, b, a * b);
                    continue;
                }
            }
        }
    }

    /*let list: Vec<u32> = vec![111111, 123321, 123123, 142351, 1100];

    for n in list.iter() {
        println!("{}  ", is_2k_digit_palindrome(&n, 3));
    }*/
}

fn is_2k_digit_palindrome(n: &u32, k: u8) -> bool {
    //println!("gamma {} {}", n, k);
    if k == 1 {
        return n % 11 == 0;
    }

    if n % 11 != 0 {
        return false;
    }

    let ten_to_the_2k_minus_1: u32 = (10u32).pow((2*k - 1) as u32) as u32;
    let first_digit = n / ten_to_the_2k_minus_1;

    if n % 10 == first_digit {
        let inner_digits = (n - first_digit * (ten_to_the_2k_minus_1 + 1)) / 10;
        return is_2k_digit_palindrome(&inner_digits, k - 1);
    } else {
        return false;
    }
}
