use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    // Key point: element in position of nCr is used nCr * 2^(15 - r) times

    let file = File::open("data/p018.txt").unwrap();
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

    for row in nums.iter() {
        for num in row.iter() {
            print!("{:4}", num);
        }
        println!("");
    }
    println!("\n");

    // max sum from nums[i][j]
    let max_sum_from = &mut nums;

    for n in (0..max_sum_from.len() - 1).rev() {
        for r in 0..=n {
            let left = max_sum_from[n + 1][r];
            let right = max_sum_from[n + 1][r + 1];
            max_sum_from[n][r] += if left > right { left } else { right };
        }
    }

    for row in max_sum_from.iter() {
        for num in row.iter() {
            print!("{:4}", num);
        }
        println!("");
    }
}
