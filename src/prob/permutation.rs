use crate::{Answer, Input};

const FACTORIAL: [usize; 8] = [1, 1, 2, 6, 24, 120, 720, 5040];

pub struct Permutations<T> {
    data: Vec<T>,
    data_len: usize,
    swaps: Vec<usize>, // tracks how many swaps we've done for each index
    swap_idx: usize, // index of the next position to swap
    first: bool,
}

impl<T> Permutations<T> {
    pub fn new(data: Vec<T>) -> Self {
        let data_len: usize = data.len();
        Self {
            data,
            data_len,
            swaps: vec![0; data_len],
            swap_idx: 0,
            first: true,
        }
    }
}

impl<T: Clone> Permutations<T> {
    fn next_permutation(&mut self) -> Option<&[T]> {
        // First yield is the initial arrangement
        if self.first {
            self.first = false;
            self.swap_idx = 1;
            return Some(&self.data);
        }

        while self.swap_idx < self.data_len {
            if self.swaps[self.swap_idx] < self.swap_idx {
                if self.swap_idx % 2 == 0 {
                    self.data.swap(0, self.swap_idx);
                } else {
                    let ci = self.swaps[self.swap_idx];
                    self.data.swap(ci, self.swap_idx);
                }
                self.swaps[self.swap_idx] += 1;
                self.swap_idx = 0;
                return Some(&self.data);
            } else {
                self.swaps[self.swap_idx] = 0;
                self.swap_idx += 1;
            }
        }
        None
    }
}

impl Iterator for Permutations<usize> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_permutation().map(|v| {
            let s = v.iter().map(|n| n.to_string()).collect::<Vec<_>>();
            s.join(" ")
        })
    }
}

pub fn run(input: Input) -> Answer {
    let n: usize = input.parse();
    assert!(n > 0);
    assert!(n <= 7);

    // cheat because n has known bounds
    let count = FACTORIAL[n];
    let p = Permutations::new((0..n).collect());

    Answer::CountAndIter {
        count,
        iter: Box::new(p),
    }
}
