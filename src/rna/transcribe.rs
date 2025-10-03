use crate::{Answer, Input};

pub fn run(dna: Input) -> Answer {
    Answer::String(dna.get().replace("T", "U"))
}
