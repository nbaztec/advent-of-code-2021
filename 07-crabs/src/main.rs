use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();
    // part_two();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut positions = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        positions = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }

    // println!("{}", calc_fuel(positions));
    println!("{}", calc_fuel_v2(positions));
}

fn calc_fuel(mut positions: Vec<usize>) -> usize {
    positions.sort();
    let median = positions[positions.len() / 2];
    let mut fuel = 0usize;
    for &p in &positions {
        fuel += if p > median { p - median } else { median - p };
    }

    fuel
}

fn calc_fuel_v2(mut positions: Vec<usize>) -> usize {
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();
    let mut min_fuel = None;
    let mut min_num = None;

    for num in min..=max {
        let mut fuel = 0usize;
        for &p in &positions {
            let n = if p > num {
                (p - num) as f32
            } else {
                (num - p) as f32
            };

            fuel += ((n * (n + 1.0)) / 2.0).round() as usize;
        }

        if let Some(mf) = min_fuel {
            if fuel < mf {
                min_fuel = Some(fuel);
                min_num = Some(num);
            }
        } else {
            min_fuel = Some(fuel);
            min_num = Some(num);
        }
    }

    min_fuel.unwrap()
}
