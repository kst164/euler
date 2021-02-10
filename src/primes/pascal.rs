// Used it once, maybe in a lib later

fn pascal() {
    let mut pascal: Vec<Vec<u16>> = vec![vec![1]];
    let num_rows = 15;

    for row_num in 1..num_rows {
        pascal.push(Vec::with_capacity(row_num + 1));
        pascal[row_num].push(1);

        for i in 1..row_num {
            let left = pascal[row_num - 1][i - 1];
            let right = pascal[row_num - 1][i];
            pascal[row_num].push(left + right);
        }

        pascal[row_num].push(1);
    }

    for row in pascal {
        println!("{:?}", row);
    }

}
