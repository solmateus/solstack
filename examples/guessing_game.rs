use rand::prelude::*;
use solstack::macros::*;
use solstack::prelude::*;

fn main() {
    let mut data = GameData::default();
    let mut stack = Stack::new();

    stack_push!(stack, data, SMenu);

    while stack.is_running() {
        stack_tick!(stack, data);
    }

    println!();
    println!("> Bye, bye; tchau, tchau!");
}

fn input() -> String {
    use std::io::Write;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("> Expected valid UTF-8 input.");
    print!("> ");
    let _ = std::io::stdout().flush();

    s.trim().to_owned()
}

#[derive(Default)]
struct GameData {
    secret: i32,
    tries: i32,
}

struct SMenu;
struct SGame;

impl State<GameData> for SMenu {
    fn on_tick(&mut self, _data: &mut GameData) -> Trans<GameData> {
        println!();
        println!("> Guessing Game");
        println!("> (p) play");
        println!("> (q) quit");

        let option = input();

        match option.to_lowercase().as_str() {
            "p" => trans_push!(SGame),
            "q" => trans_quit!(),
            _ => {
                println!();
                println!("> Error parsing input :^(");
                println!("> (p) play");
                println!("> (q) quit");
                trans_none!()
            }
        }
    }
}

impl State<GameData> for SGame {
    fn on_start(&mut self, data: &mut GameData) {
        println!();
        println!("> A random number (ranged 1..=100) was generated.");
        println!("> Guess it! You'll have tips.");
        let mut rng = thread_rng();
        data.secret = rng.gen_range(1..=100);
    }
    fn on_tick(&mut self, data: &mut GameData) -> Trans<GameData> {
        println!();
        println!("> Guess:");

        let unparsed_guess = input();

        if unparsed_guess.as_str() == "q" {
            return trans_pop!();
        }

        match unparsed_guess.parse::<i32>() {
            Ok(guess) => match guess.cmp(&data.secret) {
                std::cmp::Ordering::Less => {
                    data.tries += 1;
                    println!();
                    println!("> You guessed low!");
                    println!("> Try again!");
                    println!("> Guesses: {}.", data.tries);
                    trans_none!()
                }
                std::cmp::Ordering::Equal => {
                    data.tries += 1;
                    println!();
                    println!("> YOU DIT IT!");
                    println!("> It took you {} tries.", data.tries);
                    trans_pop!()
                }
                std::cmp::Ordering::Greater => {
                    data.tries += 1;
                    println!();
                    println!("> You guessed high!");
                    println!("> Try again!");
                    println!("> Guesses: {}.", data.tries);
                    trans_none!()
                }
            },
            Err(_) => {
                println!();
                println!("> Type a number! The secret is in range 1..=100.");
                trans_none!()
            }
        }
    }

    fn on_stop(&mut self, data: &mut GameData) {
        data.tries = 0;
    }
}
