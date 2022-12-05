const LOWER_CASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let mut input: Vec<&str> = include_str!("../input").split('\n').collect();
    input.remove(input.len() - 1);
    let mut total_prio = 0;
    for group in input.chunks(3) {
        if group.len() != 3 {
            panic!("Group size was {}", group.len());
        }
        let mutual_char = get_mutual_char(group);
        let char_index = LOWER_CASE
            .iter()
            .position(|x| x == &mutual_char.to_ascii_lowercase())
            .unwrap();
        if mutual_char.is_uppercase() {
            total_prio += char_index + 27
        } else {
            total_prio += char_index + 1
        }
    }
    println!("{}", total_prio)
}

fn get_mutual_char(group: &[&str]) -> char {
    let mut mutual_char = None;
    for character in group[0].chars() {
        if group[1].contains(character) && group[2].contains(character) {
            mutual_char = Some(character);
            break;
        }
    }
    mutual_char.expect("Characters did not match.")
}
