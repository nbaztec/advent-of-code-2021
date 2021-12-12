use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();
}

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut adjacency = HashMap::new();

    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        // println!("{}", line);
        let parts = line.split("-").collect::<Vec<_>>();
        let (a, b) = (parts[0].to_owned(), parts[1].to_owned());
        adjacency
            .entry(a.clone())
            .or_insert(vec![])
            .push(Node::new(b.clone()));
        adjacency
            .entry(b.clone())
            .or_insert(vec![])
            .push(Node::new(a.clone()));
    }

    // display(&adjacency);

    let all_paths = path_finder(&adjacency, "start", "end");
    // all_paths
    //     .iter()
    //     .for_each(|path| println!("{}", path.join(",")));

    println!("{}", all_paths.len());
}

fn part_two() {}

fn display(adjacency: &HashMap<String, Vec<Node>>) {
    adjacency
        .iter()
        .for_each(|(k, nodes)| nodes.iter().for_each(|v| println!("{} - {}", k, v.value)))
}

fn path_finder(adjacency: &HashMap<String, Vec<Node>>, start: &str, end: &str) -> Vec<Vec<String>> {
    let mut all_paths = vec![];
    let mut visited = HashSet::new();
    visited.insert(start.to_owned());

    path_finder_recur(
        adjacency,
        visited,
        String::from(""),
        start,
        end,
        vec![start.to_owned()],
        &mut all_paths,
    );

    all_paths
}

fn path_finder_recur(
    adjacency: &HashMap<String, Vec<Node>>,
    visited: HashSet<String>,
    visited_twice: String,
    cur: &str,
    end: &str,
    path: Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) {
    if cur == end {
        all_paths.push(path);
        return;
    }

    if let Some(neighbours) = adjacency.get(cur) {
        for n in neighbours {
            let mut new_visited_twice = visited_twice.clone();
            if !n.is_big_cave() && visited.contains(&n.value) {
                if visited_twice.is_empty() && !n.is_start_node() {
                    new_visited_twice = n.value.clone();
                } else {
                    continue;
                }
            }

            let mut new_visited = visited.clone();
            if !n.big {
                new_visited.insert(n.value.clone());
            }

            let mut new_path = path.clone();
            new_path.push(n.value.clone());
            path_finder_recur(
                adjacency,
                new_visited,
                new_visited_twice,
                &n.value,
                end,
                new_path,
                all_paths,
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    value: String,
    big: bool,
}

impl Node {
    fn new(value: String) -> Self {
        let big = value.chars().all(|x| x.is_uppercase());
        Node { value, big }
    }

    fn is_big_cave(&self) -> bool {
        self.big
    }

    fn is_start_node(&self) -> bool {
        self.value == "start"
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
