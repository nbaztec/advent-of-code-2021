use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();

    // let mut grid = vec![];
    let mut payload = String::from("");
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        payload = line;
    }

    let s = decode_hex(&payload);
    // println!("{:?}", s);
    let (p, sz) = parse_packet(&s);
    // println!("{:?}", p);

    // println!("{}", score_packet_version(&p));
    println!("{}", resolve_packet(&p));
}

fn score_packet_version(p: &Packet) -> usize {
    let mut score = p.version as usize;
    if let Some(sub_packets) = &p.sub_packets {
        for sp in sub_packets {
            score += score_packet_version(sp);
        }
    }
    score
}

fn resolve_packet(p: &Packet) -> usize {
    match p.packet_type {
        0 => {
            if let Some(sub_packets) = &p.sub_packets {
                let mut sum = 0usize;
                for sp in sub_packets {
                    sum += resolve_packet(sp);
                }
                return sum;
            }
            panic!("expected value")
        }
        1 => {
            if let Some(sub_packets) = &p.sub_packets {
                let mut prod = 1usize;
                for sp in sub_packets {
                    prod *= resolve_packet(sp);
                }
                return prod;
            }
            panic!("expected value")
        }
        2 => {
            if let Some(sub_packets) = &p.sub_packets {
                let mut min = usize::MAX;
                for sp in sub_packets {
                    let v = resolve_packet(sp);
                    if v < min {
                        min = v;
                    }
                }
                return min;
            }
            panic!("expected value")
        }
        3 => {
            if let Some(sub_packets) = &p.sub_packets {
                let mut max = usize::MIN;
                for sp in sub_packets {
                    let v = resolve_packet(sp);
                    if v > max {
                        max = v;
                    }
                }
                return max;
            }
            panic!("expected value")
        }
        4 => {
            if let Some(v) = &p.value {
                return *v;
            }
            panic!("expected value")
        }
        5 => {
            if let Some(sub_packets) = &p.sub_packets {
                let a = resolve_packet(&sub_packets[0]);
                let b = resolve_packet(&sub_packets[1]);
                return if a > b { 1 } else { 0 };
            }
            panic!("expected value")
        }
        6 => {
            if let Some(sub_packets) = &p.sub_packets {
                let a = resolve_packet(&sub_packets[0]);
                let b = resolve_packet(&sub_packets[1]);
                return if a < b { 1 } else { 0 };
            }
            panic!("expected value")
        }
        7 => {
            if let Some(sub_packets) = &p.sub_packets {
                let a = resolve_packet(&sub_packets[0]);
                let b = resolve_packet(&sub_packets[1]);
                return if a == b { 1 } else { 0 };
            }
            panic!("expected value")
        }
        _ => {
            panic!("invalid type {}", p.packet_type)
        }
    }
}

fn get_packet_values(p: &Packet) -> Vec<usize> {
    if let Some(v) = &p.value {
        return vec![*v];
    }

    if let Some(sub_packets) = &p.sub_packets {
        return sub_packets
            .iter()
            .map(|x| x.value.unwrap())
            .collect::<Vec<_>>();
    }

    panic!("no packet values");
}

fn parse_packet(s: &str) -> (Packet, usize) {
    let mut idx = 0;
    let version = u8::from_str_radix(&s[idx..idx + 3], 2).unwrap();
    idx += 3;

    let packet_type = u8::from_str_radix(&s[idx..idx + 3], 2).unwrap();
    idx += 3;

    match packet_type {
        4 => {
            // println!("parse literal packet");
            let (literal, num_bytes) = parse_literal(&s[idx..]);
            idx += num_bytes;
            return (
                Packet {
                    version,
                    packet_type,
                    value: Some(literal),
                    sub_packets: None,
                },
                idx,
            );
        }
        _ => {
            // println!("parse operator packet");
            let (sub_packets, num_bytes) = parse_operator(&s[idx..]);
            idx += num_bytes;
            (
                Packet {
                    version,
                    packet_type,
                    value: None,
                    sub_packets: Some(sub_packets),
                },
                idx,
            )
        }
    }
}

fn parse_operator(s: &str) -> (Vec<Packet>, usize) {
    let mut sub_packets = vec![];
    let mut idx = 0usize;

    match s.chars().nth(idx).unwrap() {
        '0' => {
            idx += 1;
            let len = usize::from_str_radix(&s[idx..idx + 15], 2).unwrap();
            idx += 15;

            let mut l = 0usize;
            loop {
                let (packet, num_bytes) = parse_packet(&s[idx..]);
                idx += num_bytes;
                l += num_bytes;
                // println!("{:?}", packet);
                sub_packets.push(packet);

                if l == len {
                    break;
                }
            }
        }
        '1' => {
            idx += 1;
            let count = u8::from_str_radix(&s[idx..idx + 11], 2).unwrap();
            idx += 11;

            for _ in 0..count {
                let (packet, num_bytes) = parse_packet(&s[idx..]);
                idx += num_bytes;
                // println!("{:?}", packet);
                sub_packets.push(packet);
            }
        }
        _ => panic!("invalid"),
    };

    (sub_packets, idx)
}

fn parse_literal(s: &str) -> (usize, usize) {
    let mut result = String::from("");
    let mut idx = 0usize;
    // println!("{}", s);
    loop {
        // println!("{},{} | {}", idx + 1, idx + 5, &s[idx..idx + 5]);
        let p = &s[idx + 1..idx + 5];
        result.push_str(p);

        if s.chars().nth(idx).unwrap() == '0' {
            idx += 5;
            break;
        }
        idx += 5;
    }

    (decode_bin(&result), idx)
}

fn decode_hex(s: &str) -> String {
    (0..s.len())
        .map(|i| format!("{:04b}", u8::from_str_radix(&s[i..i + 1], 16).unwrap()))
        .collect::<String>()
}

fn decode_bin(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

#[derive(Debug, Default)]
struct Packet {
    version: u8,
    packet_type: u8,
    value: Option<usize>,
    sub_packets: Option<Vec<Packet>>,
}

impl Packet {
    fn parse(s: &str) -> Self {
        Packet::default()
    }
}
