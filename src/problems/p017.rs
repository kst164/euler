pub fn run() {
    println!("{}", char_count(&111));
    let mut total: usize = 0;
    for i in 1..=1000 {
        total += char_count(&i);
    }

    println!("{}", total);
}

fn char_count(n: &usize) -> usize {
    let ones_place_lengths: [usize; 9] = [3, 3, 5, 4, 4, 3, 5, 5, 4];
    let tens_place_lengths: [usize; 9] = [3, 6, 6, 5, 5, 5, 7, 6, 6];
    let eleven_to_nineteen: [usize; 9] = [6, 6, 8, 8, 7, 7, 9, 8, 8];

    if *n == 1000 {
        return 11;
    }

    let mut char_len: usize = 0;

    if *n >= 100 {
        char_len += ones_place_lengths[n / 100 - 1] + 7; // 7 for "hundred"
    }

    if *n % 100 != 0 {
        if *n >= 100 {
            char_len += 3; // "and"
        }

        let last_digit = n % 10;
        let tens_digit = (n % 100) / 10;

        if last_digit == 0 {
            char_len += tens_place_lengths[tens_digit - 1];
        } else if tens_digit == 0 {
            char_len += ones_place_lengths[last_digit - 1];
        } else if tens_digit == 1 {
            char_len += eleven_to_nineteen[last_digit - 1];
        } else {
            char_len += tens_place_lengths[tens_digit - 1] + ones_place_lengths[last_digit - 1];
        }
    }

    char_len
}
