use std::fs;

use rand::Rng;

pub trait Chooser {
    fn guess(&self, letter: char);
    fn pattern(&self) -> &str;
    fn word(&self) -> &str;
}

pub struct EvilChooser {
    pub len_min: u32,
    pub len_max: u32,
}

pub struct RandomChooser {
    word: String
}

impl RandomChooser {
    pub fn new(file_path: &str) -> Self {
        let contents = fs::read_to_string(file_path).expect("failed to read \
        {file_path}");
        let words: Vec<&str> = contents.lines().collect();
        RandomChooser {
            word: words[rand::thread_rng().gen_range(1..words.len())].to_string()
        }
    }
}

impl Chooser for RandomChooser {
    fn guess(&self, letter: char) {
        todo!()
    }

    fn pattern(&self) -> &str {
        todo!()
    }

    fn word(& self) -> & str {
        &self.word
    }
}

impl EvilChooser {
    pub fn new(len_min: u32, len_max: u32, file_path: &str) -> Self {
        todo!()
    }
}

impl Chooser for EvilChooser {
    fn guess(&self, letter: char) {
        todo!()
    }

    fn pattern(&self) -> &'static str {
        todo!()
    }

    fn word(&self) -> &'static str {
        todo!()
    }
}
