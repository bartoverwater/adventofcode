use std::num::ParseIntError;

struct Move {
    amount: u8,
    from: u8,
    to: u8,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let parts: Vec<Result<u8, ParseIntError>> = input
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>())
            .collect();
        Move {
            amount: parts[1].clone().unwrap(),
            from: parts[3].clone().unwrap(),
            to: parts[5].clone().unwrap(),
        }
    }
}

fn main() {
    let mut stacks = vec![
        vec!['H', 'B', 'V', 'W', 'N', 'M', 'L', 'P'],
        vec!['M', 'Q', 'H'],
        vec!['N', 'D', 'B', 'G', 'F', 'Q', 'M', 'L'],
        vec!['Z', 'T', 'F', 'Q', 'M', 'W', 'G'],
        vec!['M', 'T', 'H', 'P'],
        vec!['C', 'B', 'M', 'J', 'D', 'H', 'G', 'T'],
        vec!['M', 'N', 'B', 'F', 'V', 'R'],
        vec!['P', 'L', 'H', 'M', 'R', 'G', 'S'],
        vec!['P', 'D', 'B', 'C', 'N'],
    ];
    let input = include_str!("../input");
    let mut x: Vec<&str> = input
        .split('\n')
        .collect::<Vec<&str>>()
        .drain(10..)
        .collect();
    x.remove(x.len() - 1);
    let input: Vec<Move> = x.into_iter().map(|line| line.into()).collect();
    for m in input {
        let from_stack = stacks.get_mut(usize::from(m.from - 1)).unwrap();
        let mut containers: Vec<char> = from_stack
            .drain(from_stack.len() - usize::from(m.amount)..)
            .collect();
        let to_stack = stacks.get_mut(usize::from(m.to - 1)).unwrap();
        to_stack.append(&mut containers);
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
}
