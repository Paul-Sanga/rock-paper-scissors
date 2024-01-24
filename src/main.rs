use std::io;
enum Choice{
    Rock,
    Paper,
    Scissors
}

fn get_user_input() -> Choice{
    let mut _choice: Choice;
    loop{
        println!("please enter your choice");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
            .expect("error reading from terminal");
        let input = input.trim().to_lowercase();
        match input.as_str(){
            "rock" => {
                break _choice = Choice::Rock;
            },
            "paper"=>{
                break _choice = Choice::Paper;
            },
            "scissors" => {
                break _choice = Choice::Scissors;
            },
            _ => {
                continue;
            }
        }
    }
    _choice
}

fn game_logic(player_one: Choice, player_two: Choice) -> &'static str {
    match (player_one, player_two) {
        (Choice::Paper, Choice::Rock) | (Choice::Scissors, Choice::Paper) | (Choice::Rock, Choice::Scissors) =>{
            "Player 1 won!"
        },
        (Choice::Rock, Choice::Paper) | (Choice::Paper, Choice::Scissors) | (Choice::Scissors, Choice::Rock)=>{
            "Player 2 won!"
        },
        (Choice::Paper, Choice::Paper) | (Choice::Rock, Choice::Rock) | (Choice::Scissors, Choice::Scissors) =>{
            "Draw!"
        }
    }
}

fn main(){
    let computer_choice:Choice  = Choice::Paper;
    let user_input:Choice = get_user_input();
    println!("{}", game_logic(user_input, computer_choice));
}