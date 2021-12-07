use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() {
    // part_one();
    part_two();
}

// type Board = [[BoardValue; 5]; 5];

// impl std::fmt::Display for Board {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let s = String::from("");
//         for r in 0..5 {
//             for c in 0..5 {
//                 s += format!("{} ", self[r][c]);
//             }
//         }
//         write!(f, "{}", s);
//     }
// }

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut draws = vec![];
    let mut boards = vec![];
    let mut current_board = [[BoardValue::new(0); 5]; 5];
    let mut current_board_row = 0usize;
    let mut record_board = false;
    for (line_idx, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        if line_idx == 0 {
            draws = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            continue;
        }

        if line.trim().is_empty() {
            record_board = true;
            current_board_row = 0;
            continue;
        }

        if record_board {
            line.split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(idx, value)| {
                    current_board[current_board_row][idx] = BoardValue::new(value);
                });
            current_board_row += 1;
            if current_board_row == 5 {
                boards.push(current_board.clone());
                record_board = false;
            }
        }
    }

    // println!("{:?}", draws);
    let mut score = 0usize;
    let ignore = HashSet::new();
    for d in draws {
        // println!("----------- {} --------", d);
        // display_boards(&boards);
        mark_boards(d, &mut boards);
        if let Some(indices) = check_boards(&boards, &ignore) {
            // println!("WON: {}", idx);
            let idx = indices.first().unwrap().clone();
            let win_board = &boards[idx];
            // display_board(win_board);
            score = d * calc_board(win_board);
            break;
        }
        // display_boards(&boards);
    }

    println!("{}", score);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut draws = vec![];
    let mut boards = vec![];
    let mut current_board = [[BoardValue::new(0); 5]; 5];
    let mut current_board_row = 0usize;
    let mut record_board = false;
    for (line_idx, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        if line_idx == 0 {
            draws = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            continue;
        }

        if line.trim().is_empty() {
            record_board = true;
            current_board_row = 0;
            continue;
        }

        if record_board {
            line.split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(idx, value)| {
                    current_board[current_board_row][idx] = BoardValue::new(value);
                });
            current_board_row += 1;
            if current_board_row == 5 {
                boards.push(current_board.clone());
                record_board = false;
            }
        }
    }

    // println!("{:?}", draws);
    // exit(1);
    let mut remaining_boards = boards.len();
    let mut last_win_board_idx = None;
    let mut current_num = 0usize;
    let mut ignore = HashSet::new();
    let mut score = 0usize;
    for d in draws {
        current_num = d;
        // println!("----------- {} --------", d);
        // display_boards(&boards);
        mark_boards(d, &mut boards);
        if let Some(indices) = check_boards(&boards, &ignore) {
            // println!("WON: {:?} / {}", indices, remaining_boards);
            let idx = indices.first().unwrap().clone();
            last_win_board_idx = Some(idx);
            remaining_boards -= indices.len();
            indices.iter().for_each(|&idx| {
                ignore.insert(idx);
            })

            // let win_board = &boards[last_win_board_idx.unwrap()];
            // display_board(win_board);
            // break;
        }
        if remaining_boards == 0 {
            break;
        }
        // display_boards(&boards);
    }

    let win_board = &boards[last_win_board_idx.unwrap()];
    score = current_num * calc_board(win_board);

    println!("{}", score);

    for (idx, board) in boards.iter().enumerate() {
        if !ignore.contains(&idx) {
            println!("> {} {}", idx, check_board(board));
            display_board(board);
            break;
        }
    }
}

fn mark_boards(num: usize, boards: &mut Vec<[[BoardValue; 5]; 5]>) {
    for board in boards {
        for r in 0..5 {
            for c in 0..5 {
                board[r][c].mark_if(num)
            }
        }
    }
}

fn display_boards(boards: &Vec<[[BoardValue; 5]; 5]>) {
    for board in boards {
        display_board(board);
    }
}

fn display_board(board: &[[BoardValue; 5]; 5]) {
    let mut s = String::from("");
    for r in 0..5 {
        for c in 0..5 {
            s = format!("{}{}", s, board[r][c]);
        }
        s = format!("{}\n", s);
    }
    println!("{}", s);
}

fn calc_board(board: &[[BoardValue; 5]; 5]) -> usize {
    let mut sum = 0usize;
    for r in 0..5 {
        for c in 0..5 {
            let v = board[r][c];
            if !v.is_marked() {
                sum += v.num;
            }
        }
    }
    sum
}

fn check_boards(boards: &Vec<[[BoardValue; 5]; 5]>, ignore: &HashSet<usize>) -> Option<Vec<usize>> {
    let mut wins = vec![];
    for (idx, board) in boards.iter().enumerate() {
        if ignore.contains(&idx) {
            continue;
        }

        if check_board(board) {
            wins.push(idx);
        }
    }

    if wins.is_empty() {
        None
    } else {
        Some(wins)
    }
}

fn check_board(board: &[[BoardValue; 5]; 5]) -> bool {
    for row in 0..5 {
        let mut all_marked = true;
        for col in 0..5 {
            all_marked = all_marked && board[row][col].is_marked();
        }
        if all_marked {
            return true;
        }
    }

    for row in 0..5 {
        let mut all_marked = true;
        for col in 0..5 {
            all_marked = all_marked && board[col][row].is_marked();
        }
        if all_marked {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone, Copy)]
struct BoardValue {
    num: usize,
    marked: bool,
}

impl BoardValue {
    fn new(num: usize) -> Self {
        BoardValue { num, marked: false }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn mark_if(&mut self, num: usize) {
        if self.num == num {
            self.marked = true;
        }
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

impl std::fmt::Display for BoardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.marked {
            write!(f, "{}* ", self.num)
        } else {
            write!(f, "{}_ ", self.num)
        }
    }
}
