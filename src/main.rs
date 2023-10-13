use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use hangman::chooser::Chooser;

fn file_to_words(file_name: &str) -> HashSet<String> {
    fs::read_to_string(file_name).expect("failed to read file").lines().map(|x| x.to_string()).collect()
}

fn game_loop(is_evil: bool, word_to_guess: &str) {
    let mut guessed = hangman::all_underscores(word_to_guess);
    let mut remaining_guesses = word_to_guess.len();
    let mut wrong_guesses = String::new();

    loop {
        println!("\n{remaining_guesses} guesses remaining.");
        println!("Wrong guesses: {wrong_guesses}\n");
        println!("{}", guessed);
        println!("Please input your guess: ");

        let guess = hangman::char_input();
        if guessed.contains(guess) || wrong_guesses.contains(guess) {
            println!("You already guessed {guess}!");
        } else if word_to_guess.contains(guess) {
            println!("Nice! The word contains {guess}!");
            guessed = hangman::guess_char(&guessed, word_to_guess, guess);
        } else {
            println!("Sorry, the word does not contain {guess}!");
            wrong_guesses.push(guess);
            remaining_guesses -= 1;
        }

        if !guessed.contains('_') {
            println!("Congrats, you won! The word was {word_to_guess}.");
            break;
        } else if remaining_guesses <= 0 {
            println!("Sorry, you ran out of guesses! The word was {word_to_guess}.");
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
    let long_words_file_name = "google-10000-english-usa-no-swears-long.txt";
    let mut word_set = file_to_words(long_words_file_name);

    let med_words_file_name = "google-10000-english-usa-no-swears-medium.txt";
    let med_word_set = file_to_words(med_words_file_name);

    word_set.extend(med_word_set);

    let args: Vec<String> = env::args().collect();
    let chooser = hangman::chooser::RandomChooser::new(long_words_file_name);
    let word_to_guess = chooser.word();
    // evil version
    let len_min = 5;
    let len_max = 18; // "telecommunications"

    game_loop(true, word_to_guess);
    println!("\n Enter anything to quit...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line.");
}
