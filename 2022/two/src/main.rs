#[derive(PartialEq, Eq)]
pub enum Values {
    ROCK,
    PAPER,
    SCISSORS,
}

impl From<&str> for Values {
    fn from(str_val: &str) -> Self {
        if str_val == "A" {
            Values::ROCK
        } else if str_val == "B" {
            Values::PAPER
        } else if str_val == "C" {
            Values::SCISSORS
        } else {
            panic!("Invalid input value {}", str_val)
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Outcome {
    WIN,
    LOSE,
    DRAW,
}

impl From<&str> for Outcome {
    fn from(str_val: &str) -> Self {
        match str_val {
            "X" => Outcome::LOSE,
            "Y" => Outcome::DRAW,
            "Z" => Outcome::WIN,
            _ => panic!("Invalid outcome input: {}", str_val),
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let games: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();
    let mut total_score = 0;
    for game in games {
        let oppo_choice: Values = game[0].into();
        let outcome: Outcome = game[1].into();
        total_score += match oppo_choice {
            Values::ROCK => match outcome {
                Outcome::DRAW => choice_to_score(Values::ROCK),
                Outcome::LOSE => choice_to_score(Values::SCISSORS),
                Outcome::WIN => choice_to_score(Values::PAPER),
            },
            Values::SCISSORS => match outcome {
                Outcome::DRAW => choice_to_score(Values::SCISSORS),
                Outcome::LOSE => choice_to_score(Values::PAPER),
                Outcome::WIN => choice_to_score(Values::ROCK),
            },
            Values::PAPER => match outcome {
                Outcome::DRAW => choice_to_score(Values::PAPER),
                Outcome::LOSE => choice_to_score(Values::ROCK),
                Outcome::WIN => choice_to_score(Values::SCISSORS),
            },
        };
        total_score += match outcome {
            Outcome::DRAW => 3,
            Outcome::LOSE => 0,
            Outcome::WIN => 6,
        };
    }
    println!("{}", total_score);
}

fn choice_to_score(choice: Values) -> u32 {
    match choice {
        Values::ROCK => 1,
        Values::PAPER => 2,
        Values::SCISSORS => 3,
    }
}
