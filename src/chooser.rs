use std::fs;

use rand::Rng;

pub trait Chooser {
    fn guess(&mut self, letter: char);
    fn pattern(&self) -> &str;
    fn word(&self) -> &str;
}

pub struct EvilChooser {
    pub len_min: u32,
    pub len_max: u32,
}

pub struct RandomChooser {
    word: String,
    pattern: String,
}

// https://stackoverflow.com/questions/27318076/edit-string-in-place-with-a-function
fn all_underscores(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    for _ in s.chars() {
        r.push('_');
    }
    r
}

impl RandomChooser {
    pub fn new(file_path: &str) -> Self {
        let contents = fs::read_to_string(file_path).expect("failed to read \
        {file_path}");
        let words: Vec<&str> = contents.lines().collect();
        let word =
            words[rand::thread_rng().gen_range(1..words.len())].to_string();
        let pattern = all_underscores(&word);
        RandomChooser {
            word,
            pattern,
        }
    }
}

impl Chooser for RandomChooser {
    fn guess(&mut self, letter: char) {
        let mut r = String::with_capacity(self.word.len());
        for (i, c) in self.word.char_indices() {
            r.push(if c == letter { c } else { self.pattern.chars().nth(i).unwrap() });
        }
        self.pattern = r;
    }

    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn word(&self) -> &str {
        &self.word
    }
}

impl EvilChooser {
    pub fn new(len_min: u32, len_max: u32, file_path: &str) -> Self {
        todo!()
    }
}

impl Chooser for EvilChooser {
    fn guess(&mut self, letter: char) {
        todo!()
    }

    fn pattern(&self) -> &str {
        todo!()
    }

    fn word(&self) -> &str {
        todo!()
    }
}
