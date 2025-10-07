pub mod dna;
pub mod prob;
pub mod rna;
pub mod search;

use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;

pub struct Input(String);

impl Input {
    pub fn new(input: &str) -> Self {
        Self(input.trim().to_owned())
    }

    pub fn from_file(path: &Path) -> Self {
        Self(std::fs::read_to_string(path).unwrap().trim().to_owned())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn split(&self) -> (&str, &str) {
        self.0.split_once('\n').unwrap()
    }

    pub fn lines(&self) -> Vec<&str> {
        self.0.split('\n').collect()
    }

    pub fn parse<E: Debug, T: FromStr<Err = E>>(&self) -> T {
        self.0.parse().unwrap()
    }
}

pub enum Answer {
    String(String),
    Int(usize),
    StringVec(Vec<String>),
    IntVec(Vec<usize>),
    CountAndIter {
        count: usize,
        iter: Box<dyn Iterator<Item = String>>,
    },
}

impl Answer {
    pub fn print(self) {
        match self {
            Answer::String(s) => println!("{}", s),
            Answer::Int(i) => println!("{}", i),
            Answer::StringVec(v) => println!("{}", v.join(" ")),
            Answer::IntVec(v) => println!(
                "{}",
                v.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Answer::CountAndIter { count, iter } => {
                println!("{}", count);
                for s in iter {
                    println!("{}", s);
                }
            }
        }
    }
}
