use crate::{Answer, Input};

pub fn run(dna: Input) -> Answer {
    let rc = dna
        .get()
        .chars()
        .rev()
        .fold(String::with_capacity(dna.len()), |mut rc, c| {
            let complement = match c {
                'A' => 'T',
                'a' => 't',
                'T' => 'A',
                't' => 'a',
                'C' => 'G',
                'c' => 'g',
                'G' => 'C',
                'g' => 'c',
                _ => c,
            };
            rc.push(complement);
            rc
        });
    Answer::String(rc)
}
