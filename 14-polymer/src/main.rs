use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut sets = HashMap::new();
    let mut rules = HashMap::new();
    let mut phase = 0usize;
    let mut initial_chars = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        if phase == 0 {
            if line.is_empty() {
                phase += 1;
                continue;
            }
            initial_chars = line.chars().map(String::from).collect::<Vec<_>>();

            for c in initial_chars.windows(2) {
                *sets.entry(format!("{}{}", c[0], c[1])).or_insert(0usize) += 1;
            }
        } else {
            let parts = line.split(" -> ").map(String::from).collect::<Vec<_>>();
            rules.insert(parts[0].clone(), parts[1].clone());
        }
    }

    // println!("{:?} rules={} sets={}", rules, rules.len(), sets.len());
    // display(&sets);

    let mut counts = HashMap::new();
    for c in &initial_chars {
        *counts.entry(c.clone()).or_insert(0usize) += 1;
    }
    // println!("{:?}", counts);

    for _i in 0..40 {
        // println!("---> {}", _i + 1);
        polymerize(&mut sets, &rules, &mut counts);
    }

    let min = counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let max = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    // println!("{:?}", counts);
    // println!("min:{:?}  max:{:?}", min, max);

    let score = max.1 - min.1;
    println!("{}", score);
}

fn polymerize(
    sets: &mut HashMap<String, usize>,
    rules: &HashMap<String, String>,
    counts: &mut HashMap<String, usize>,
) {
    let mut new_sets = HashMap::new();
    for (k, &v) in sets.iter() {
        if let Some(c) = rules.get(k) {
            assert!(v > 0);

            *counts.entry(c.clone()).or_insert(0) += v;
            let inc_key_1 = format!("{}{}", k.chars().nth(0).unwrap(), c);
            let inc_key_2 = format!("{}{}", c, k.chars().nth(1).unwrap());

            *new_sets.entry(inc_key_1.clone()).or_insert(0) += v;
            *new_sets.entry(inc_key_2.clone()).or_insert(0) += v;
        } else {
            new_sets.entry(k.clone()).or_insert(v);
        }
    }

    *sets = new_sets;
}

fn display(sets: &HashMap<String, usize>) {
    sets.iter().for_each(|(k, v)| {
        if *v != 0 {
            println!("{} -> {}", k, v)
        }
    })
}
