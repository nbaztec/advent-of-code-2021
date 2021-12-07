use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut majority_bits = [0i32; 12];
    for line in io::BufReader::new(file).lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            let value = match c {
                '0' => -1,
                '1' => 1,
                _ => panic!("unexpected binary digit: {}", c),
            };
            majority_bits[i] += value;
        }
    }

    let gamma_bits_str = majority_bits
        .iter()
        .map(|&x| if x > 0 { "1" } else { "0" })
        .collect::<Vec<_>>()
        .join("");
    let epsilon_bits_str = majority_bits
        .iter()
        .map(|&x| if x > 0 { "0" } else { "1" })
        .collect::<Vec<_>>()
        .join("");

    let gamma = u64::from_str_radix(&gamma_bits_str, 2).unwrap();
    let epsilon = u64::from_str_radix(&epsilon_bits_str, 2).unwrap();

    println!("{}", gamma * epsilon);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut majority_bits = [0i32; 12];
    let lines = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut gamma_vec = vec![];
    let mut epsilon_vec = vec![];
    for i in 0..1 {
        let mut majority_sum = 0i32;
        for l in &lines {
            let v = match l.chars().nth(i).unwrap() {
                '0' => -1,
                '1' => 1,
                _ => panic!("invalid binary char {}", l),
            };
            majority_sum += v;
        }

        let majority = if majority_sum > 0 { '1' } else { '0' };
        for l in &lines {
            let c = l.chars().nth(i).unwrap();
            if c == majority {
                gamma_vec.push(l.clone())
            } else {
                epsilon_vec.push(l.clone())
            }
        }
    }
    // println!("{} {} / {}", gamma_vec.len(), epsilon_vec.len(), lines.len());

    for i in 1..12 {
        let mut majority_sum = 0i32;
        for l in &gamma_vec {
            let v = match l.chars().nth(i).unwrap() {
                '0' => -1,
                '1' => 1,
                _ => panic!("invalid binary char {}", l),
            };
            majority_sum += v;
        }

        let majority = if majority_sum > 0 || majority_sum == 0 {
            '1'
        } else {
            '0'
        };
        let mut next_vec = vec![];
        for l in &gamma_vec {
            let c = l.chars().nth(i).unwrap();
            if c == majority {
                next_vec.push(l.clone())
            }
        }
        // println!("{}| {} -> {} : {:?} {:?}", i, gamma_vec.len(), next_vec.len(), gamma_vec, next_vec);
        gamma_vec = next_vec;


        if gamma_vec.len() == 1 {
            break
        }
    }

    // println!("{:?}", gamma_vec);
    // exit(0);

    for i in 1..12 {
        let mut majority_sum = 0i32;
        for l in &epsilon_vec {
            let v = match l.chars().nth(i).unwrap() {
                '0' => -1,
                '1' => 1,
                _ => panic!("invalid binary char {}", l),
            };
            majority_sum += v;
        }

        let majority = if majority_sum > 0 || majority_sum == 0 {
            '0'
        } else {
            '1'
        };
        let mut next_vec = vec![];
        for l in &epsilon_vec {
            let c = l.chars().nth(i).unwrap();
            if c == majority {
                next_vec.push(l.clone())
            }
        }
        // println!("{}| {} -> {} : {:?} {:?}", i, epsilon_vec.len(), next_vec.len(), epsilon_vec, next_vec);
        epsilon_vec = next_vec;

        if epsilon_vec.len() == 1 {
            break
        }
    }

    // println!("{:?}", epsilon_vec);

    let gamma = u64::from_str_radix(&gamma_vec.join("").as_str(), 2).unwrap();
    let epsilon = u64::from_str_radix(&epsilon_vec.join("").as_str(), 2).unwrap();

    println!("{}", gamma * epsilon);
}
