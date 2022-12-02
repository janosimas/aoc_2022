use std::fs;

#[derive(PartialEq)]
enum Opt {
    Rock,
    Paper,
    Scissor,
}

impl Opt {
    pub fn value(&self) -> i32 {
        match self {
            Opt::Rock => 1,
            Opt::Paper => 2,
            Opt::Scissor => 3,
        }
    }

    pub fn draw(&self) -> Self {
        match self {
            Opt::Rock => Opt::Rock,
            Opt::Paper => Opt::Paper,
            Opt::Scissor => Opt::Scissor,
        }
    }

    pub fn win(&self) -> Self {
        match self {
            Opt::Rock => Opt::Paper,
            Opt::Paper => Opt::Scissor,
            Opt::Scissor => Opt::Rock,
        }
    }

    pub fn lose(&self) -> Self {
        match self {
            Opt::Rock => Opt::Scissor,
            Opt::Paper => Opt::Rock,
            Opt::Scissor => Opt::Paper,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    for round in contents.split('\n') {
        let (their, mine) = round.split_once(' ').unwrap();
        let their = match their {
            "A" => Opt::Rock,
            "B" => Opt::Paper,
            "C" => Opt::Scissor,
            _ => unreachable!(),
        };

        let mine = match mine {
            "X" => their.lose(),
            "Y" => their.draw(),
            "Z" => their.win(),
            _ => unreachable!(),
        };

        let result = match (&mine, &their) {
            (Opt::Rock, Opt::Scissor) | (Opt::Paper, Opt::Rock) | (Opt::Scissor, Opt::Paper) => 6,
            (Opt::Rock, Opt::Rock) | (Opt::Paper, Opt::Paper) | (Opt::Scissor, Opt::Scissor) => 3,
            (Opt::Rock, Opt::Paper) | (Opt::Paper, Opt::Scissor) | (Opt::Scissor, Opt::Rock) => 0,
        };

        total += result + mine.value();
    }

    dbg!(&total);
}
