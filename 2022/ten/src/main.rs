fn main() {
    let mut input = include_str!("../input").split('\n').collect::<Vec<&str>>();
    input.remove(input.len() - 1);
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut addx: Option<i32> = None;
    let mut indx = 0;
    while indx < input.len() {
        cycle += 1;
        if [register, register + 1, register + 2].contains(&cycle) {
            print!("#");
        } else {
            print!(".");
        }
        if cycle % 40 == 0 {
            println!();
            cycle = 0;
        }
        if let Some(v) = addx {
            register += v;
            indx += 1;
            addx = None;
        } else if input[indx] != "noop" {
            addx = Some(input[indx].split(' ').last().unwrap().parse().unwrap());
        } else {
            indx += 1;
        }
    }
}
