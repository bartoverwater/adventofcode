fn main() {
    let input = include_str!("../input");
    let mut lines: Vec<&str> = input.split('\n').collect();
    lines.remove(lines.len() - 1);
    let chars: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut highest_score = 0;
    for (row_indx, row) in chars.iter().enumerate() {
        for (col_indx, col) in row.iter().enumerate() {
            if row_indx == chars.len() - 1
                || row_indx == 0
                || col_indx == 0
                || col_indx == row.len() - 1
            {
                continue;
            }
            let mut left_score = 0;
            let mut right_score = 0;
            let mut top_score = 0;
            let mut bot_score = 0;
            for i in (0..col_indx).rev() {
                left_score += 1;
                if &row[i] >= col {
                    break;
                }
            }
            for i in col_indx + 1..row.len() {
                right_score += 1;
                if &row[i] >= col {
                    break;
                }
            }
            for i in (0..row_indx).rev() {
                top_score += 1;
                if &chars[i][col_indx] >= col {
                    break;
                }
            }
            for i in row_indx + 1..chars.len() {
                bot_score += 1;
                if &chars[i][col_indx] >= col {
                    break;
                }
            }
            let score = top_score * bot_score * left_score * right_score;
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    println!("{}", highest_score);
}
