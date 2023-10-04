// https://stackoverflow.com/questions/27318076/edit-string-in-place-with-a-function
pub fn all_underscores(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    for _ in s.chars() {
        r.push('_');
    }
    r
}

pub fn guess_char(guessed: &str, correct_word: &str, guess: char) -> String {
    let mut r = String::with_capacity(guessed.len());
    for (i, c) in correct_word.char_indices() {
        r.push(if c == guess { c } else { guessed.chars().nth(i).unwrap() });
    }
    r
}

pub fn char_input() -> char {
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

