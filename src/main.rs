use std::io;

// https://stackoverflow.com/questions/27318076/edit-string-in-place-with-a-function
fn all_underscores(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    for _ in s.chars() {
        r.push('_');
    }
    r
}

fn main() {
    let word_to_guess = "crustacean";
    let guessed = all_underscores(word_to_guess);

    loop {
        println!("{}", guessed);
        println!("Please input your guess: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim();

        if trimmed_input.len() != 1 {
            println!("Please only enter one character.")
        } else if trimmed_input == word_to_guess {
            println!("Hooray, you guessed the correct word!");
            break;
        } else {
            println!("Sorry, {trimmed_input} is not the correct word!");
        }
    }
}
