use std::fs;

#[derive(Debug, Copy, Clone)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn from_char(symb: &str) -> Self {
        match symb {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("whyyyyy"),
        }
    }

    fn from_line(line: &String) -> Self {
        let symbs: Vec<&str> = line.split(" ").collect();
        let outcome_symb: &str = symbs[1];
        Self::from_char(outcome_symb)
    }
}

impl Sign {
    fn from_char(symb: &str) -> Self {
        match symb {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("whyyyyy"),
        }
    }

    fn other(&self, outcome: &Outcome) -> Self {
        match self {
            Sign::Rock => match outcome {
                Outcome::Win => Sign::Paper,
                Outcome::Draw => Sign::Rock,
                Outcome::Loss => Sign::Scissors,
            },

            Sign::Paper => match outcome {
                Outcome::Win => Sign::Scissors,
                Outcome::Draw => Sign::Paper,
                Outcome::Loss => Sign::Rock,
            },

            Sign::Scissors => match outcome {
                Outcome::Win => Sign::Rock,
                Outcome::Draw => Sign::Scissors,
                Outcome::Loss => Sign::Paper,
            },
        }
    }
}

struct Game {
    you: Sign,
    opponent: Sign,
}

impl Game {
    fn from_line(line: &String) -> Self {
        let symbs: Vec<&str> = line.split(" ").collect();
        let (you_symb, opp_symb): (&str, &str) = (symbs[1], symbs[0]);
        Self {
            you: Sign::from_char(you_symb),
            opponent: Sign::from_char(opp_symb),
        }
    }

    fn game_score(&self) -> i32 {
        match self.you {
            Sign::Rock => {
                1 + match self.opponent {
                    Sign::Rock => 3,
                    Sign::Paper => 0,
                    Sign::Scissors => 6,
                }
            }
            Sign::Paper => {
                2 + match self.opponent {
                    Sign::Rock => 6,
                    Sign::Paper => 3,
                    Sign::Scissors => 0,
                }
            }
            Sign::Scissors => {
                3 + match self.opponent {
                    Sign::Rock => 0,
                    Sign::Paper => 6,
                    Sign::Scissors => 3,
                }
            }
        }
    }

    // fn game_outcome(&self) -> Outcome { }
}

fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

fn total_score(games: &Vec<Game>) -> i32 {
    let score: i32 = games.iter().fold(0, |acc, x| acc + x.game_score());
    score
}

fn main() {
    println!("Hello, world!");
    let filepath = "./input.txt";
    let sep = "\n";
    let input: Vec<String> = read_input(filepath, sep);
    let games: Vec<Game> = input.iter().map(|l| Game::from_line(l)).collect();
    let score = total_score(&games);
    println!("Total score is: {score}");

    let opponent_signs: Vec<Sign> = games
        .iter()
        .map(|x| x.opponent) // this only works because of derive[Copy]
        .collect();
    let desired_outcomes: Vec<Outcome> = input.iter().map(|l| Outcome::from_line(l)).collect();
    let signs_to_play: Vec<Sign> = opponent_signs.iter().zip(desired_outcomes.iter()).map(|(s, o)| {
        s.other(&o)
    }).collect();
    let fixed_games: Vec<Game> = opponent_signs.iter().zip(signs_to_play.iter()).map(|(opp, you)| {
        Game {
            you: *you,
            opponent: *opp
        }
    }).collect();
    let fixed_score = total_score(&fixed_games);
    println!("Fixed games total score is: {fixed_score}");
}
