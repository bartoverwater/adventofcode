fn main() {
    let input = include_str!("../input");

    let mut highest_val: [(usize, u32); 3] = [(0, 0), (0, 0), (0, 0)];
    let mut current_elf_values: Vec<u32> = vec![];
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() {
            let total_val = calculate_total(&current_elf_values);
            let lowest_val = highest_val
                .iter()
                .reduce(|x, y| if x.1 < y.1 { x } else { y })
                .unwrap();
            if total_val > lowest_val.1 {
                let lowest_val_index = highest_val.iter().position(|x| x == lowest_val).unwrap();
                highest_val[lowest_val_index] = (index, total_val);
            }
            current_elf_values = vec![];
        } else {
            current_elf_values.push(line.parse::<u32>().unwrap());
        }
    }
    let mut total = 0;
    let mut elves = Vec::new();
    for (i, val) in highest_val {
        elves.push(i);
        total += val;
    }
    println!("{}, elves: {:?}", total, elves);
}

fn calculate_total(all_values: &[u32]) -> u32 {
    all_values.iter().sum()
}
