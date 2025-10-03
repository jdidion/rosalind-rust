use crate::{Answer, Input};

pub fn run(input: Input) -> Answer {
    let counts = input
        .get()
        .chars()
        .map(|b| match b {
            'A' | 'a' => 0,
            'C' | 'c' => 1,
            'G' | 'g' => 2,
            'T' | 't' => 3,
            _ => unreachable!("{}", b),
        })
        .fold([0; 4], |mut acc, i| {
            acc[i] += 1;
            acc
        })
        .iter()
        .map(|count| count.to_string())
        .collect::<Vec<_>>();

    Answer::Vec(counts)
}
