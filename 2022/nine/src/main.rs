use std::collections::HashSet;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn go(&self, pos: (i32, i32)) -> (i32, i32) {
        match *self {
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Up => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0, pos.1 - 1),
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => unreachable!(),
        }
    }
}

struct Motion {
    direction: Direction,
    count: i32,
}

impl From<&str> for Motion {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(' ').collect();
        Motion {
            direction: split[0].into(),
            count: split[1].parse().unwrap(),
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let mut input: Vec<&str> = input.split('\n').collect();
    input.remove(input.len() - 1);
    let moves: Vec<Motion> = input.into_iter().map(|s| s.into()).collect();
    let mut tail_positions = HashSet::new();
    let mut positions = [(0, 0); 10];
    for m in moves {
        for _ in 0..m.count {
            positions[0] = m.direction.go(positions[0]);
            for i in 1..positions.len() {
                if let Some(pos) = move_adjacent(positions[i], positions[i - 1]) {
                    positions[i] = pos;
                } else {
                    break;
                }
            }
            tail_positions.insert(positions[positions.len() - 1]);
        }
    }
    println!("{}", tail_positions.len())
}

fn move_adjacent(tail: (i32, i32), head: (i32, i32)) -> Option<(i32, i32)> {
    let dx = tail.0 - head.0;
    let dy = tail.1 - head.1;

    if (dx == 2 || dx == -2) && (dy == 2 || dy == -2) {
        Some((head.0 + dx.clamp(-1, 1), head.1 + dy.clamp(-1, 1)))
    } else if dx == 2 || dx == -2 {
        Some((head.0 + dx.clamp(-1, 1), head.1))
    } else if dy == 2 || dy == -2 {
        Some((head.0, head.1 + dy.clamp(-1, 1)))
    } else {
        None
    }
}
