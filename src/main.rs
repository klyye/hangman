use std::io;
use rand::Rng;

// https://stackoverflow.com/questions/27318076/edit-string-in-place-with-a-function
fn all_underscores(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    for _ in s.chars() {
        r.push('_');
    }
    r
}

fn guess_char(guessed: &str, correct_word: &str, guess: char) -> String {
    let mut r = String::with_capacity(guessed.len());
    for (i, c) in correct_word.char_indices() {
        r.push(if c == guess { c } else { guessed.chars().nth(i).unwrap() });
    }
    r
}

fn char_input() -> char {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    while input.trim().len() != 1 {
        println!("Please only enter one character.");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
    input.trim().to_lowercase().chars().nth(0).unwrap()
}

fn main() {
    let word_list = ["crustacean", "masterpiece", "experience", "override", "dynamic", "system", "finished", "reference", "moisturize"];
    let word_to_guess = word_list[rand::thread_rng().gen_range(1..=word_list.len() - 1)];
    let mut guessed = all_underscores(word_to_guess);
    let mut remaining_guesses = word_to_guess.len();

    loop {
        println!("\n{}", guessed);
        println!("{remaining_guesses} guesses remaining.\n");
        println!("Please input your guess: ");

        let guess = char_input();
        if word_to_guess.contains(guess) {
            println!("Nice! The word contains {guess}!");
            guessed = guess_char(&guessed, word_to_guess, guess);
        } else {
            println!("Sorry, the word does not contain {guess}!");
            remaining_guesses -= 1;
        }

        if !guessed.contains('_') {
            println!("Congrats, you won!");
            break;
        } else if remaining_guesses <= 0 {
            println!("Sorry, you ran out of guesses! The word was {word_to_guess}.");
            break;
        }
    }
}
