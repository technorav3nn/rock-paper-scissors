mod rps;

use rps::Rps;
use rps::RpsChoices;

use inquire::{error::InquireError, Select};

fn main() {
    
    let game = Rps::new();
    let options: Vec<&str> = vec!["Rock", "Paper", "Scissors"];
    
    let ans: Result<&str, InquireError> = Select::new("Choose your choice for Rock Paper Scissors!", options).prompt();

    match ans {
        Ok(choice) => {
            let rps_choice = match choice {
                "Rock" => RpsChoices::Rock,
                "Paper" => RpsChoices::Paper,
                "Scissors" => RpsChoices::Scissors,
                _ => RpsChoices::Scissors
            };
            let (result, info) = game.start(rps_choice);
            let result_with_player_name = if result.contains("tie") || result.contains("Computer") {
                result.to_string()
            } else {
                format!("{} {}", "Colin", result)
            };
            println!("{}", result_with_player_name);
            println!("Info: {}", info);
        },
        Err(_) => println!("There was an error, please try again"),
    }
}