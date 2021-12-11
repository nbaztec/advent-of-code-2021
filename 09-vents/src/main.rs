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
    let mut grid = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        grid.push(
            line.split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut threat = 0usize;
    for row in 0..grid.len() {
        let row_top = if row == 0 { None } else { Some(&grid[row - 1]) };
        let row_bot = if row == grid.len() - 1 {
            None
        } else {
            Some(&grid[row + 1])
        };
        let row_cur = &grid[row];

        for i in 0..row_cur.len() {
            let v = row_cur[i];
            let mut neighbours = vec![];
            if i != 0 {
                neighbours.push(row_cur[i - 1]) // left
            }
            if i != row_cur.len() - 1 {
                neighbours.push(row_cur[i + 1]) // right
            }
            if let Some(row_top) = row_top {
                neighbours.push(row_top[i]) // top
            }
            if let Some(row_bot) = row_bot {
                neighbours.push(row_bot[i]) // bot
            }

            let &min = neighbours.iter().min().unwrap();
            if v < min {
                // println!("{} {},{}", v, row, i);
                threat += v + 1;
            }
        }
    }

    println!("{}", threat);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut visited = vec![];
    let mut grid = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let nums = line
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        visited.push(nums.iter().map(|_| 0).collect::<Vec<_>>());
        grid.push(nums);
    }

    boundary_fill_begin(&mut grid, &mut visited);

    let mut groups = HashMap::new();
    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            let key = visited[i][j];
            if key == 0 {
                continue;
            }

            *groups.entry(key).or_insert(0) += 1;
        }
    }

    // println!("{:?}", visited);
    // println!("{:?}", groups);
    let mut sorted_lengths = groups.into_values().collect::<Vec<_>>();
    sorted_lengths.sort();
    let score = sorted_lengths
        .iter()
        .rev()
        .take(3)
        .cloned()
        .reduce(|acc, item| acc * item)
        .unwrap();

    println!("{}", score);
}

fn boundary_fill_begin(grid: &mut [Vec<usize>], visited: &mut [Vec<usize>]) {
    let mut group = 1usize;

    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] == 0 && grid[i][j] != 9 {
                boundary_fill(grid, visited, i, j, group);
                group += 1;
            }
        }
    }
}

fn boundary_fill(
    grid: &mut [Vec<usize>],
    visited: &mut [Vec<usize>],
    row: usize,
    col: usize,
    group: usize,
) {
    if grid[row][col] == 9 {
        return;
    }
    visited[row][col] = group;

    // left
    if col != 0 && visited[row][col - 1] == 0 {
        boundary_fill(grid, visited, row, col - 1, group);
    }
    // right
    if col != grid[row].len() - 1 && visited[row][col + 1] == 0 {
        boundary_fill(grid, visited, row, col + 1, group);
    }
    // top
    if row != 0 && visited[row - 1][col] == 0 {
        boundary_fill(grid, visited, row - 1, col, group);
    }
    // bot
    if row != grid.len() - 1 && visited[row + 1][col] == 0 {
        boundary_fill(grid, visited, row + 1, col, group);
    }
}
