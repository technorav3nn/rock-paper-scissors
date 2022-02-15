use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RpsChoices {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

impl std::fmt::Display for RpsChoices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RpsChoices::Rock => write!(f, "Rock"),
            RpsChoices::Paper => write!(f, "Paper"),
            RpsChoices::Scissors => write!(f, "Scissors"),
        }
    }
}

pub struct Rps {}

impl Rps {
    pub const fn new() -> Self {
        return Rps {};
    }
}

impl Rps {
    fn get_random_choice(&self) -> RpsChoices {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => RpsChoices::Rock,
            1 => RpsChoices::Paper,
            2 => RpsChoices::Scissors,
            _ => RpsChoices::Rock
        }
    }
    pub fn start(&self, player_choice: RpsChoices) -> (&'static str, String) {
        let computer_choice = &self.get_random_choice();

        if computer_choice == &player_choice {
            return ("Its a tie", format!("You chose {}, Computer Chose {}", player_choice.to_string(), computer_choice.to_string()));
        }
        let result = match (player_choice, computer_choice) {
            (RpsChoices::Rock, RpsChoices::Scissors) => "wins!",
            (RpsChoices::Paper, RpsChoices::Rock) => "wins!",
            (RpsChoices::Scissors, RpsChoices::Paper) => "wins!",
            _ => "Computer wins!"
        };
        return (result, format!("You chose {}, Computer Chose {}", player_choice.to_string(), computer_choice.to_string()))
    }
}