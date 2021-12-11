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
        let nums = line
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        grid.push(nums);
    }

    let mut total = 0usize;
    for _day in 0..100 {
        // println!("day {}", _day + 1);
        let count = step(&mut grid);
        total += count;
        // display(&mut grid);
        // println!("{:?} {} ({})", grid, count, total);
    }

    println!("{}", total);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut grid = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let nums = line
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        grid.push(nums);
    }

    let mut all_flashed_at = 0usize;
    for day in 0..1000 {
        let count = step(&mut grid);
        // println!("day {}: {}", day + 1, count);
        if count == 100 {
            all_flashed_at = day + 1;
            break;
        }
    }

    println!("{}", all_flashed_at);
}

fn _display(grid: &mut [Vec<usize>]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{} ", grid[i][j]);
        }
        println!("");
    }
    println!("");
}

fn step(grid: &mut [Vec<usize>]) -> usize {
    let mut flashed = grid
        .iter()
        .map(|x| x.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] += 1;
        }
    }

    loop {
        let mut none_flashed = true;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 && !flashed[i][j] {
                    flashed[i][j] = true;
                    flash(grid, i, j);
                    none_flashed = false;
                }
            }
        }
        // println!("> {}", none_flashed);
        // display(grid);
        // sleep(Duration::from_secs(2));

        if none_flashed {
            break;
        }
    }

    let mut count = 0usize;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] > 9 {
                grid[i][j] = 0;
                count += 1;
            }
        }
    }

    count
}

fn flash(grid: &mut [Vec<usize>], row: usize, col: usize) {
    // left
    if col != 0 {
        let c = col - 1;
        grid[row][c] += 1;
        // top-left
        if row != 0 {
            grid[row - 1][c] += 1;
        }
        // bottom-left
        if row != grid.len() - 1 {
            grid[row + 1][c] += 1;
        }
    }
    // right
    if col != grid[row].len() - 1 {
        let c = col + 1;
        grid[row][c] += 1;
        // top-right
        if row != 0 {
            grid[row - 1][c] += 1;
        }
        // bottom-right
        if row != grid.len() - 1 {
            grid[row + 1][c] += 1;
        }
    }
    // top
    if row != 0 {
        grid[row - 1][col] += 1;
    }
    // bot
    if row != grid.len() - 1 {
        grid[row + 1][col] += 1;
    }
}
