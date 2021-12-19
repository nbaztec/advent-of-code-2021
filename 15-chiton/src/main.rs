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

    let mut grid = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let row = line
            .chars()
            .map(String::from)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        grid.push(row);
    }

    // display(&grid);
    let mut grid = expand_grid(grid);
    // display(&grid);

    let mut graph = new_graph(&grid);

    let origin = to_key(0, 0);
    let destination = to_key(grid.len() - 1, grid[0].len() - 1);

    // println!("{:#?}", graph);
    let risk = dijkstra(&mut graph, origin, destination);

    println!("{}", risk);
}

fn expand_grid(grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let new_dim_row = grid.len() * 5;
    let new_dim_col = grid[0].len() * 5;

    let mut new_grid = vec![vec![0usize; new_dim_col]; new_dim_row];
    for i in 0..5 {
        for j in 0..5 {
            let mut g = grid.clone();
            increment_grid(&mut g, i + j);
            for ai in 0..g.len() {
                for aj in 0..g[ai].len() {
                    new_grid[ai + g.len() * i][aj + g.len() * j] = g[ai][aj];
                }
            }
        }
    }

    new_grid
}

fn increment_grid(grid: &mut [Vec<usize>], value: usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let n = (grid[i][j] + value);
            grid[i][j] = if n > 9 { 1 + n % 10 } else { n };
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    cost: usize,
    name: &'a str,
}

impl<'a> State<'a> {
    fn new(cost: usize, name: &'a str) -> Self {
        State { cost, name }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    graph: &mut HashMap<String, HashMap<String, usize>>,
    origin: String,
    destination: String,
) -> usize {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    let mut queue = BinaryHeap::new();

    for v in graph.keys() {
        let v = v.as_str();
        queue.push(State::new(usize::MAX, v));
        dist.insert(v, usize::MAX);
        prev.insert(v, "");
    }

    dist.insert(&origin, 0);
    queue.push(State::new(0, &origin));

    while !queue.is_empty() {
        let State {
            cost,
            name: min_vertex,
        } = queue.pop().unwrap();

        // println!("min {} ({:?})", min_vertex, cost);
        if min_vertex == destination.as_str() {
            break;
        }

        // println!(
        //     "min {} ({}) {:?}",
        //     min_vertex,
        //     dist[min_vertex],
        //     dist.values()
        //         .filter(|x| **x != usize::MAX)
        //         .collect::<Vec<_>>(),
        // );

        for (neighbor, weight) in &graph[min_vertex] {
            let neighbor = neighbor.as_str();
            let next = State::new(cost + weight, neighbor);
            if next.cost < dist[neighbor] {
                queue.push(next);
                *dist.get_mut(neighbor).unwrap() = next.cost;
                *prev.get_mut(neighbor).unwrap() = min_vertex;
            }
        }
    }

    let mut path = vec![];
    let mut u = destination.as_str();
    if !prev[u].is_empty() || u == &origin {
        while !u.is_empty() {
            path.insert(0, u.clone());
            u = &prev[u];
        }
    }

    // println!("{:?}", dist);
    // println!("{:?}", path);

    dist[destination.as_str()]
}

fn display(grid: &[Vec<usize>]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if j % 10 == 0 {
                print!("  ");
            }
            print!("{} ", grid[i][j]);
        }
        println!("");
    }
    println!("");
}

fn new_graph(grid: &[Vec<usize>]) -> HashMap<String, HashMap<String, usize>> {
    let mut graph = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            add_node(&mut graph, &grid, i, j)
        }
    }

    graph
}

fn add_node(
    graph: &mut HashMap<String, HashMap<String, usize>>,
    grid: &[Vec<usize>],
    row: usize,
    col: usize,
) {
    let key = to_key(row, col);
    // println!("add {}", key);
    let src = graph.entry(key).or_default();

    // left
    if col != 0 {
        add_edge(src, grid, row, col - 1);
    }
    // right
    if col != grid[row].len() - 1 {
        add_edge(src, grid, row, col + 1);
    }
    // top
    if row != 0 {
        add_edge(src, grid, row - 1, col);
    }
    // bot
    if row != grid.len() - 1 {
        add_edge(src, grid, row + 1, col);
    }
}

fn to_key(row: usize, col: usize) -> String {
    format!("{},{}", row, col)
}

fn add_edge(src: &mut HashMap<String, usize>, grid: &[Vec<usize>], row: usize, col: usize) {
    let key = to_key(row, col);
    let value = grid[row][col];
    // println!("\tadd {} {}", key, value);
    src.insert(key, value);
}
