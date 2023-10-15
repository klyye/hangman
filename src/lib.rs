use std::io;

pub mod chooser;

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

