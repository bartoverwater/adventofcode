struct Range {
    start: u32,
    end: u32,
}

fn main() {
    let input = include_str!("../input");
    let mut split: Vec<&str> = input.split('\n').collect();
    split.remove(split.len() - 1);
    let ranges: Vec<Vec<Range>> = split
        .iter()
        .map(|line| {
            line.split(',')
                .map(|r| {
                    let s: Vec<&str> = r.split('-').collect();
                    Range {
                        start: s[0].parse().unwrap(),
                        end: s[1].parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();
    let mut amount = 0;
    for pair in ranges {
        let first = &pair[0];
        let second = &pair[1];
        if first.start <= second.end && first.end >= second.start {
            println!(
                "{}-{}, {}-{}",
                first.start, first.end, second.start, second.end
            );
            amount += 1;
        }
    }
    println!("{}", amount)
}
