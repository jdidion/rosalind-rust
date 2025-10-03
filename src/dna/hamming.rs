use crate::{Answer, Input};

pub fn run(input: Input) -> Answer {
    let (s1, s2) = input.split();
    let dist = s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count();
    Answer::Int(dist)
}
