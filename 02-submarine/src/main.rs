use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut x = 0usize;
    let mut y = 0usize;
    for line in io::BufReader::new(file).lines() {
        let cmd = line
            .unwrap()
            .split(' ')
            .map(String::from)
            .collect::<Vec<_>>();
        let direction = cmd[0].as_str();
        let units = cmd[1].parse::<usize>().unwrap();

        match direction {
            "forward" => {
                x += units;
            }
            "up" => {
                y -= units;
            }
            "down" => {
                y += units;
            }
            _ => panic!("invalid direction '{}'", direction),
        }
    }

    println!("{}", x * y);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut x = 0usize;
    let mut y = 0usize;
    let mut aim = 0usize;
    for line in io::BufReader::new(file).lines() {
        let cmd = line
            .unwrap()
            .split(' ')
            .map(String::from)
            .collect::<Vec<_>>();
        let direction = cmd[0].as_str();
        let units = cmd[1].parse::<usize>().unwrap();

        match direction {
            "forward" => {
                x += units;
                y += aim * units;
            }
            "up" => {
                aim -= units;
            }
            "down" => {
                aim += units;
            }
            _ => panic!("invalid direction '{}'", direction),
        }
    }

    println!("{}", x * y);
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
