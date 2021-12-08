use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();
    // part_two();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut buckets = [0usize; 9];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        line.split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .for_each(|x| {
                buckets[x] += 1;
            });
    }

    // println!("{:?}", buckets);
    for i in 0..256 {
        next_day(&mut buckets);
        // println!("{:?}", buckets);
    }

    println!("{}", total_fishes(&mut buckets));
}

fn part_two() {}

fn next_day(buckets: &mut [usize; 9]) {
    let spawned_new = buckets[0];
    for i in 0..8 {
        buckets[i] = buckets[i + 1];
    }
    buckets[8] = spawned_new;
    buckets[6] += spawned_new;
}

fn total_fishes(buckets: &mut [usize; 9]) -> usize {
    let mut sum = 0usize;
    for i in 0..9 {
        sum += buckets[i];
    }
    sum
}
