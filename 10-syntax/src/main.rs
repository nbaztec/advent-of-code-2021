use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut inputs = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        inputs.push(
            line.split("")
                .filter(|x| !x.is_empty())
                .map(String::from)
                .collect::<Vec<_>>(),
        );
    }

    let mut inverse = HashMap::new();
    inverse.insert("(", ")");
    inverse.insert("{", "}");
    inverse.insert("[", "]");
    inverse.insert("<", ">");
    // inverse.insert(")", "(");
    // inverse.insert("}", "{");
    // inverse.insert("]", "[");
    // inverse.insert(">", "<");

    let mut points = HashMap::new();
    points.insert(")", 3usize);
    points.insert("]", 57);
    points.insert("}", 1197);
    points.insert(">", 25137);

    let mut score = 0usize;
    for (_idx, input) in inputs.iter().enumerate() {
        let mut stack = vec![];
        let mut invalid_token = None;
        for (_col, token) in input.iter().enumerate() {
            match token.as_str() {
                "(" | "[" | "{" | "<" => stack.push(token.clone()),
                ")" | "]" | "}" | ">" => {
                    let popped = stack.pop().unwrap();
                    if token.as_str() != inverse[popped.as_str()] {
                        invalid_token = Some(token.to_string());
                        break;
                    }
                }
                _ => panic!("invalid token"),
            }
        }

        if let Some(invalid_token) = invalid_token {
            score += points[invalid_token.as_str()];
        }
    }

    println!("{}", score);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut inputs = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        inputs.push(
            line.split("")
                .filter(|x| !x.is_empty())
                .map(String::from)
                .collect::<Vec<_>>(),
        );
    }

    let mut inverse = HashMap::new();
    inverse.insert("(", ")");
    inverse.insert("{", "}");
    inverse.insert("[", "]");
    inverse.insert("<", ">");
    // inverse.insert(")", "(");
    // inverse.insert("}", "{");
    // inverse.insert("]", "[");
    // inverse.insert(">", "<");

    let mut points = HashMap::new();
    points.insert(")", 1usize);
    points.insert("]", 2);
    points.insert("}", 3);
    points.insert(">", 4);

    let mut scores = vec![];
    for (_idx, input) in inputs.iter().enumerate() {
        let mut stack = vec![];
        let mut is_bad = false;
        for (_col, token) in input.iter().enumerate() {
            match token.as_str() {
                "(" | "[" | "{" | "<" => stack.push(token.clone()),
                ")" | "]" | "}" | ">" => {
                    let popped = stack.pop().unwrap();
                    if token.as_str() != inverse[popped.as_str()] {
                        // invalid_token = Some(token.to_string());
                        // break;
                        is_bad = true;
                        break;
                    }
                }
                _ => panic!("invalid token"),
            }
        }
        if is_bad {
            continue;
        }

        let mut cur_score = 0usize;
        while !stack.is_empty() {
            let ch = stack.pop().unwrap();
            let req = inverse[ch.as_str()];
            cur_score = cur_score * 5 + points[req];
        }

        scores.push(cur_score);
        // println!("{:?}", stack);
    }
    scores.sort();
    println!("{:?}", scores);
    println!("{:?} {}", scores.len(), scores.len() / 2);
    let score = scores[scores.len() / 2];

    println!("{}", score);
}
