const TOTAL: usize = 200;
const COIN_COUNT: usize = 8;

const COIN_VALUES: [usize; COIN_COUNT] = [200, 100, 50, 20, 10, 5, 2, 1];
//const COIN_VALUES: [usize; COIN_COUNT] = [10, 5, 2, 1];

pub fn run() {
    let mut coin_counts = [0; COIN_COUNT];

    println!("{}", count(TOTAL, 0, &mut coin_counts));
}



fn count(in_hand: usize, depth: usize, coin_counts: &mut [usize]) -> usize {
    if depth == COIN_COUNT - 1 {
        coin_counts[depth] = in_hand;
        //println!("{:?}", coin_counts);
        return 1;
    }
    let mut this_count = 0;

    for coin_count_at_this_depth in 0..=(in_hand / COIN_VALUES[depth]) {
        coin_counts[depth] = coin_count_at_this_depth;

        let now_in_hand = in_hand - COIN_VALUES[depth] * coin_count_at_this_depth;

        this_count += count(now_in_hand, depth + 1, coin_counts);
    }

    coin_counts[depth] = 0;

    this_count
}


