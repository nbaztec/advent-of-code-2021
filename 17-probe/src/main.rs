use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();

    let mut grid = Grid::default();
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        for (part_idx, part) in line
            .unwrap()
            .replace("target area: ", "")
            .split(",")
            .enumerate()
        {
            if part_idx == 0 {
                let range = part
                    .trim()
                    .replace("x=", "")
                    .split("..")
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                grid.x = (range[0], range[1]);
            } else {
                let range = part
                    .trim()
                    .replace("y=", "")
                    .split("..")
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                grid.y = (range[0], range[1]);
            }
        }
    }

    // println!("{:?}", grid);
    let (max_height, distinct) = guess_velocity(&grid);

    println!("{}", max_height);
    println!("{}", distinct);
}

fn guess_velocity(grid: &Grid) -> (isize, usize) {
    let mut all_velocities = vec![];
    let mut best_velocity = (0, XY::new(0, 0));
    for vx in 0..=grid.x.1 {
        for vy in grid.y.0..=200 {
            let velocity = XY::new(vx, vy);
            // let velocity = XY::new(7, -1);
            // print!("{}, ", velocity);
            let mut probe = Probe::new(velocity.clone());

            loop {
                // println!("{:?}", probe);
                if grid.failed(&probe) {
                    // println!(".. failure");
                    break;
                }

                probe.step();
                // grid.debug_contains(&probe);
                if grid.contains(&probe) {
                    // println!("velocity: {} - {}", velocity, probe.max_height);
                    // grid.debug_contains(&probe);
                    // println!(".. success");
                    all_velocities.push(velocity.clone());

                    if probe.max_height > best_velocity.0 {
                        best_velocity = (probe.max_height, velocity);
                    }
                    break;
                }
            }

            // return (0, 0);
        }
    }
    // println!("best: {:?}", best_velocity);
    // println!("all: {:?}", all_velocities);

    // let s = "23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
    // 25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
    // 8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
    // 26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
    // 20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
    // 25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
    // 25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
    // 8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
    // 24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
    // 7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
    // 23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
    // 27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
    // 8,-2    27,-8   30,-5   24,-7";
    // let all_points = s
    //     .split_whitespace()
    //     .filter(|x| !x.is_empty())
    //     .map(|s| {
    //         let v = s
    //             .split(",")
    //             .map(|x| x.parse::<isize>().unwrap())
    //             .collect::<Vec<_>>();
    //         XY::new(v[0], v[1])
    //     })
    //     .for_each(|p| {
    //         if !all_velocities.contains(&p) {
    //             println!("missing {}", p);
    //         }
    //     });

    // println!("count: {:?}", all_velocities.len());
    (best_velocity.0, all_velocities.len())
}

#[derive(Default, Debug, Clone)]
struct Probe {
    position: XY,
    velocity: XY,
    max_height: isize,
}

impl Probe {
    fn new(velocity: XY) -> Self {
        Probe {
            velocity,
            ..Default::default()
        }
    }

    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y > self.max_height {
            self.max_height = self.position.y;
        }

        if self.velocity.x > 0 {
            self.velocity.x -= 1;
        } else if self.velocity.x < 0 {
            self.velocity.x += 1;
        }

        self.velocity.y -= 1;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct XY {
    x: isize,
    y: isize,
}

impl XY {
    fn new(x: isize, y: isize) -> Self {
        XY { x, y }
    }
}

impl std::fmt::Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Default, Clone)]
struct Grid {
    x: (isize, isize),
    y: (isize, isize),
}

impl Grid {
    fn contains(&self, p: &Probe) -> bool {
        let x_valid = (self.x.0..=self.x.1).contains(&p.position.x);
        let y_valid = (self.y.0..=self.y.1).contains(&p.position.y);
        x_valid && y_valid
    }

    fn debug_contains(&self, p: &Probe) {
        println!(
            "{}..={} ? {} = {} | {}..={} ? {} = {}",
            self.x.0,
            self.x.1,
            &p.position.x,
            (self.x.0..=self.x.1).contains(&p.position.x),
            self.y.0,
            self.y.1,
            &p.position.y,
            (self.y.0..=self.y.1).contains(&p.position.y)
        );
    }

    fn failed(&self, p: &Probe) -> bool {
        // overshot
        if p.position.x > self.x.1 {
            return true;
        }
        if p.position.y < self.y.0 {
            return true;
        }

        // not enough X velocity
        if p.velocity.x == 0 && p.position.x < self.x.0 {
            return true;
        }

        false
    }
}
