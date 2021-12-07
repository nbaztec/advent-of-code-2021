use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    //    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut prev: Option<usize> = None;
    let mut inc = 0usize;
    for line in io::BufReader::new(file).lines() {
        let num = line.unwrap().parse::<usize>().unwrap();
        if prev.is_some() {
            if let Some(p) = prev {
                if p < num {
                    inc += 1;
                }
            }
        }
        prev = Some(num);
    }

    println!("{}", inc);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut prev_sum: Option<usize> = None;
    let mut inc = 0usize;
    let mut window = Window::new();
    for line in io::BufReader::new(file).lines() {
        let num = line.unwrap().parse::<usize>().unwrap();
        window.push(num);
        if window.full() {
            let current = window.value();
            if prev_sum.is_some() {
                if let Some(prev) = prev_sum {
                    if prev < current {
                        inc += 1;
                    }
                }
            }
            prev_sum = Some(current);
        }
    }

    println!("{}", inc);
}

struct Window {
    nums: [usize; 3],
    idx: usize,
    count: usize,
}

impl Window {
    fn new() -> Self {
        Window {
            nums: [0, 0, 0],
            idx: 0,
            count: 0,
        }
    }
    fn push(&mut self, n: usize) {
        self.nums[self.idx] = n;
        self.idx = (self.idx + 1) % 3;
        self.count += 1;
    }

    fn full(&self) -> bool {
        self.count >= 3
    }

    fn value(&self) -> usize {
        self.nums.iter().sum()
    }
}
