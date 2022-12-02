use crate::Outcome::{Draw, Loose, Win};
use crate::Shape::{Paper, Rock, Scissors};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn new_from_char(chr: char) -> Self {
        match chr {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("Invalid char!")
        }
    }
    fn score(&self) -> u8 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

enum Outcome {
    Win,
    Loose,
    Draw,
}

impl Outcome {
    fn new_from_char(chr: char) -> Self {
        match chr {
            'X' => Loose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Invalid char!")
        }
    }
    fn score(&self) -> u8 {
        match self {
            Win => 6,
            Draw => 3,
            Loose => 0
        }
    }
}

struct Round {
    my_shape: Shape,
    enemy_shape: Shape,
}

impl Round {
    fn new_from_line(line: &str) -> Self {
        let mut chars = line.chars();
        let enemy_shape = Shape::new_from_char(chars.next().unwrap());
        chars.next(); // space between the characters
        let my_choice = Outcome::new_from_char(chars.next().unwrap());
        let my_shape = Round::target_shape(&enemy_shape, &my_choice);
        Round {
            enemy_shape,
            my_shape,
        }
    }

    fn target_shape(enemy_choice: &Shape, desired_outcome: &Outcome) -> Shape {
        match (enemy_choice, desired_outcome) {
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Loose) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Draw) => Paper,
            (Paper, Loose) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Draw) => Scissors,
            (Scissors, Loose) => Paper
        }
    }

    fn outcome(&self) -> Outcome {
        match (&self.my_shape, &self.enemy_shape) {
            (Rock, Rock) => Draw,
            (Paper, Paper) => Draw,
            (Scissors, Scissors) => Draw,
            (Rock, Scissors) => Win,
            (Scissors, Paper) => Win,
            (Paper, Rock) => Win,
            _ => Loose
        }
    }

    fn score(&self) -> u8 {
        self.outcome().score() + self.my_shape.score()
    }
}

fn main() {
    let input_str = include_str!("../assets/day2.txt");
    let rounds = input_str.lines()
        .map(Round::new_from_line)
        .fold(0u32, |acc, curr| curr.score() as u32 + acc);
    println!("{:?}", rounds);
}