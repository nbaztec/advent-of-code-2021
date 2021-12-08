use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() {
    part_one();
    // part_two();
}

const FIELD_SIZE: usize = 1000;

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut field = [[0u8; FIELD_SIZE]; FIELD_SIZE];
    let mut vents = vec![];
    for (line_idx, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let mut a = (0, 0);
        let mut b = (0, 0);
        for (idx, v) in line.split(" ").enumerate() {
            match idx {
                0 => {
                    let p = v
                        .split(",")
                        .map(|x| x.parse::<i16>().unwrap())
                        .collect::<Vec<_>>();
                    a = (p[0], p[1]);
                }
                2 => {
                    let p = v
                        .split(",")
                        .map(|x| x.parse::<i16>().unwrap())
                        .collect::<Vec<_>>();
                    b = (p[0], p[1]);
                }
                _ => (),
            };
        }
        if true || a.0 == b.0 || a.1 == b.1 {
            vents.push(Vent::new(a, b));
        }
    }

    // println!("{:?}", vents);
    for v in vents {
        // println!("{:?}", v);
        for (x, y) in v.points() {
            // println!("{}, {}", y, x);
            field[y as usize][x as usize] += 1;
        }
        // display_field(&field);
        // break;
    }

    // display_field(&field);
    println!("{}", check_field(&field));
}

fn part_two() {}

fn display_field(field: &[[u8; FIELD_SIZE]; FIELD_SIZE]) {
    let mut s = String::from("");
    for r in 0..FIELD_SIZE {
        for c in 0..FIELD_SIZE {
            let v = if field[r][c] == 0 { ".".to_string() } else { field[r][c].to_string() };
            s = format!("{}{}", s, v);
        }
        s = format!("{}\n", s);
    }

    println!("{}", s);
}

fn check_field(field: &[[u8; FIELD_SIZE]; FIELD_SIZE]) -> usize {
    let mut count = 0usize;
    for r in 0..FIELD_SIZE {
        for c in 0..FIELD_SIZE {
            if field[r][c] > 1 {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug)]
struct Vent {
    a: (i16, i16),
    b: (i16, i16),
}

impl Vent {
    fn new(a: (i16, i16), b: (i16, i16)) -> Self {
        Vent { a, b }
    }

    fn points(&self) -> Vec<(usize, usize)> {
        let x_inc = match self.a.0 - self.b.0 {
            0 => 0i16,
            v if v > 0 => -1,
            v if v < 0 => 1,
            _ => panic!("unexpected difference"),
        };
        let y_inc = match self.a.1 - self.b.1 {
            0 => 0i16,
            v if v > 0 => -1,
            v if v < 0 => 1,
            _ => panic!("unexpected difference"),
        };

        let mut pts = vec![(self.a.0 as usize, self.a.1 as usize)];

        if x_inc == 0 || y_inc == 0 {
            let mut x = self.a.0 as i16;
            while x != self.b.0 as i16 {
                x += x_inc;
                pts.push((x as usize, self.a.1 as usize));
            }

            let mut y = self.a.1 as i16;
            while y != self.b.1 as i16 {
                y += y_inc;
                pts.push((self.a.0 as usize, y as usize));
            }
        } else {
            let mut x = self.a.0 as i16;
            let mut y = self.a.1 as i16;
            while x != self.b.0 && y != self.b.1 {
                x += x_inc;
                y += y_inc;
                pts.push((x as usize, y as usize));
            }
        }

        // pts.push((self.b.0 as usize, self.b.1 as usize));

        pts
    }
}
