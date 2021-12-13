use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut phase = 0usize;
    let mut coordinates = vec![];
    let mut instructions = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        if phase == 0 {
            if line.is_empty() {
                phase += 1;
                continue;
            }
            let parts = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            coordinates.push((parts[0], parts[1]));
        } else {
            let parts = line
                .replace("fold along ", "")
                .split("=")
                .map(String::from)
                .collect::<Vec<_>>();
            instructions.push((parts[0].clone(), parts[1].parse::<usize>().unwrap()));
        }
    }

    let mut x_max = coordinates
        .iter()
        .max_by(|(a_x, _), (b_x, _)| a_x.cmp(b_x))
        .map(|v| v.0 + 1)
        .unwrap();
    let mut y_max = coordinates
        .iter()
        .max_by(|(_, a_y), (_, b_y)| a_y.cmp(b_y))
        .map(|v| v.1 + 1)
        .unwrap();
    // println!("{} {}", x_max, y_max);

    let mut grid = vec![vec![false; x_max]; y_max];
    for (x, y) in &coordinates {
        grid[*y][*x] = true;
    }
    // println!("{:?}", instructions);

    // display(&grid, x_max, y_max);
    for (direction, index) in instructions {
        if direction.as_str() == "x" {
            fold_on_x(&mut grid, x_max, y_max, index);
            x_max = index;
        } else {
            fold_on_y(&mut grid, x_max, y_max, index);
            y_max = index;
        }

        // println!("");
        // println!("--> {}={}", direction, index);
        // display(&grid, x_max, y_max);
        // break;
    }
    display(&grid, x_max, y_max);
    println!("{}", score(&grid, x_max, y_max));
}

fn display(grid: &Vec<Vec<bool>>, x_max: usize, y_max: usize) {
    for y in 0..y_max {
        for x in 0..x_max {
            print!("{}", if grid[y][x] { "#" } else { "." })
        }
        println!("");
    }
}

fn score(grid: &Vec<Vec<bool>>, x_max: usize, y_max: usize) -> usize {
    let mut score = 0usize;
    for y in 0..y_max {
        for x in 0..x_max {
            if grid[y][x] {
                score += 1;
            }
        }
    }

    score
}

fn fold_on_x(grid: &mut Vec<Vec<bool>>, x_max: usize, y_max: usize, index: usize) {
    for y in 0..y_max {
        for x in (index + 1)..x_max {
            let new_x = 2 * index - x;
            grid[y][new_x] = grid[y][x] || grid[y][new_x]
        }
    }
}

fn fold_on_y(grid: &mut Vec<Vec<bool>>, x_max: usize, y_max: usize, index: usize) {
    for y in (index + 1)..y_max {
        for x in 0..x_max {
            let new_y = 2 * index - y;
            grid[new_y][x] = grid[y][x] || grid[new_y][x]
        }
    }
}
