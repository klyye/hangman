use std::collections::HashSet;
use rand::Rng;

pub trait Chooser {
    fn choose_word(&self) -> &str;
}

pub struct EvilChooser {

}

pub struct RandomChooser {}

impl Chooser for EvilChooser {
    fn choose_word(&self) -> &str {
        todo!()
    }
}

impl Chooser for RandomChooser {
    fn choose_word(&self) -> &str {
        todo!()
    }
}