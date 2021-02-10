pub fn run() {
    let mut max_length = 1;
    let mut max_at = 1;
    for i in 1..10000000 {
        let length = get_length(&i);
        if length > max_length {
            max_length = length;
            max_at = i;
        }
    }

    println!("{:?}", (max_at, max_length));
}

fn get_length(n: &u64) -> u64 {
    let mut chain_length: u64 = 1;
    let mut val = *n;

    while val != 1 {
        if val % 2 == 0 {
            val /= 2;
        } else {
            val = 3 * val + 1;
        }
        chain_length += 1;
    }

    chain_length
}
