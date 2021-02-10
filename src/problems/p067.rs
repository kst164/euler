use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    // LITERALLY JUST COPIED p018.rs ITS THE SAME THING

    let file = File::open("data/p067.txt").unwrap();
    let reader = BufReader::new(file);

    let mut nums: Vec<Vec<u16>> = vec![];

    for (row_num, line_res) in reader.lines().enumerate() {
        let line = line_res.unwrap();
        nums.push(Vec::with_capacity(row_num + 1));

        for col in (0..line.len()).step_by(3) {
            let num = line[col..(col + 2)].parse().unwrap();
            nums[row_num].push(num);
        }
    }

    // max sum from nums[i][j]
    let max_sum_from = &mut nums;

    for n in (0..max_sum_from.len() - 1).rev() {
        for r in 0..=n {
            let left = max_sum_from[n + 1][r];
            let right = max_sum_from[n + 1][r + 1];
            max_sum_from[n][r] += if left > right { left } else { right };
        }
    }

    println!("{}", max_sum_from[0][0]);
}
