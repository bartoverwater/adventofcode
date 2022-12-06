use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let mut buffer: [Option<char>; 14] = [None; 14];
    for (i, character) in input.char_indices() {
        buffer.rotate_left(1);
        buffer[13] = Some(character);
        if i >= 13 && buffer.iter().collect::<HashSet<_>>().len() == 14 {
            println!("{}", i + 1);
            break;
        }
    }
}
