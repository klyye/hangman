use std::env;
use std::io;

use hangman::chooser::Chooser;

fn game_loop(is_evil: bool, words_file: &str) {
    let mut chooser = hangman::chooser::RandomChooser::new(words_file);
    let mut remaining_guesses = chooser.word().len();
    let mut wrong_guesses = String::new();

    loop {
        println!("\n{remaining_guesses} guesses remaining.");
        println!("Wrong guesses: {wrong_guesses}\n");
        println!("{}", chooser.pattern());
        println!("Please input your guess: ");

        let guess = hangman::char_input();
        if chooser.pattern().contains(guess) || wrong_guesses.contains(guess) {
            println!("You already guessed {guess}!");
        } else if chooser.word().contains(guess) {
            println!("Nice! The word contains {guess}!");
            chooser.guess(guess);
        } else {
            println!("Sorry, the word does not contain {guess}!");
            wrong_guesses.push(guess);
            remaining_guesses -= 1;
        }

        let word = chooser.word();
        if !chooser.pattern().contains('_') {
            // why can't i use chooser.word()?
            println!("Congrats, you won! The word was {word}.");
            break;
        } else if remaining_guesses <= 0 {
            println!("Sorry, you ran out of guesses! The word was {word}.");
            break;
        }
    }
}

/*
    TODO:
    - refactor to make it cleaner
    - add command line argument to enable "evil" mode
    - add evil mode
    - add unit tests
*/
fn main() {
    let words_file_name = "google-10000.txt";

    let args: Vec<String> = env::args().collect();

    // evil version
    let len_min = 5;
    let len_max = 18; // "telecommunications"

    game_loop(true, words_file_name);
    println!("\n Enter anything to quit...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line.");
}
