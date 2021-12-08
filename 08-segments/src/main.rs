use std::array::IntoIter;
use std::collections::hash_map::RandomState;
use std::collections::hash_set::Iter;
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

fn part_one() {
    let file = File::open("input.txt").unwrap();
    let mut signals = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let parts = line.trim().split("|").collect::<Vec<_>>();
        let input = parts[0]
            .trim()
            .split(" ")
            .map(String::from)
            .collect::<Vec<_>>();
        let output = parts[1]
            .trim()
            .split(" ")
            .map(String::from)
            .collect::<Vec<_>>();
        signals.push((input, output));
    }

    // println!("{:?}", signals);
    let mut sum = 0usize;
    for (_, output) in signals {
        for o in output {
            match o.len() {
                7 | 2 | 4 | 3 => sum += 1, // 8 | 1 | 4 | 7
                _ => (),
            }
            // println!("{:?} {}", output, sum);
        }
    }
    println!("{}", sum);
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let mut signals = vec![];
    for (_, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let parts = line.trim().split("|").collect::<Vec<_>>();
        let input = parts[0]
            .trim()
            .split(" ")
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(String::from)
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        let output = parts[1]
            .trim()
            .split(" ")
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(String::from)
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        signals.push((input, output));
        // break;
    }

    let A = String::from("a");
    let B = String::from("b");
    let C = String::from("c");
    let D = String::from("d");
    let E = String::from("e");
    let F = String::from("f");
    let G = String::from("g");

  

    let mut total = 0usize;
    for (inputs, outputs) in &signals {
        let mut wiring = HashMap::new();
        wiring.insert(&A, "".to_string());
        wiring.insert(&B, "".to_string());
        wiring.insert(&C, "".to_string());
        wiring.insert(&D, "".to_string());
        wiring.insert(&E, "".to_string());
        wiring.insert(&F, "".to_string());
        wiring.insert(&G, "".to_string());
    
        let mut wiring_remain = vec![
            A.clone(),
            B.clone(),
            C.clone(),
            D.clone(),
            E.clone(),
            F.clone(),
            G.clone(),
        ]
        .into_iter()
        .collect::<HashSet<_>>();

        let num_1 = inputs.iter().find(|x| x.len() == 2).unwrap();
        let num_7 = inputs.iter().find(|x| x.len() == 3).unwrap();
        let num_4 = inputs.iter().find(|x| x.len() == 4).unwrap();
        let num_8 = inputs.iter().find(|x| x.len() == 7).unwrap();
        let mut num_3 = &HashSet::new();
        let mut num_9 = &HashSet::new();
        let mut num_0 = &HashSet::new();
        let mut num_2 = &HashSet::new();
        let mut num_5 = &HashSet::new();
        let mut num_6 = &HashSet::new();

        let map_a = num_7.difference(num_1).collect::<HashSet<_>>();
        update_wiring(&mut wiring, &mut wiring_remain, &A, map_a);

        for inp in inputs.iter().filter(|x| x.len() == 5) {
            if inp.intersection(num_7).collect::<Vec<_>>().len() == 3 {
                num_3 = inp;
                let h = num_7.union(num_4).cloned().collect::<HashSet<_>>();
                let map_g = inp.difference(&h).collect::<HashSet<_>>();
                update_wiring(&mut wiring, &mut wiring_remain, &G, map_g);

                let set_g = vec![wiring[&G].clone()].into_iter().collect::<HashSet<_>>();
                let h = num_7.union(&set_g).cloned().collect::<HashSet<_>>();
                let map_d = num_3.difference(&h).collect::<HashSet<_>>();
                update_wiring(&mut wiring, &mut wiring_remain, &D, map_d);

                break;
            }
        }

        for inp in inputs.iter().filter(|x| x.len() == 6) {
            if num_3.is_subset(inp) {
                num_9 = inp;
                let map_b = inp.difference(num_3).collect::<HashSet<_>>();
                update_wiring(&mut wiring, &mut wiring_remain, &B, map_b);
                break;
            }
        }

        let map_e = num_8.difference(num_9).collect::<HashSet<_>>();
        update_wiring(&mut wiring, &mut wiring_remain, &E, map_e);

        let h_2 = vec![
            wiring[&A].clone(),
            wiring[&D].clone(),
            wiring[&E].clone(),
            wiring[&G].clone(),
        ]
        .into_iter()
        .collect::<HashSet<_>>();

        for inp in inputs.iter().filter(|x| x.len() == 5) {
            if h_2.is_subset(inp) {
                num_2 = inp;
                let map_c = inp.difference(&h_2).collect::<HashSet<_>>();
                update_wiring(&mut wiring, &mut wiring_remain, &C, map_c);
            }
        }

        // println!("remain: {:?}", wiring_remain);

        assert!(wiring_remain.len() == 1);
        let elem_val = wiring_remain.iter().next().unwrap().to_string();
        wiring_remain.remove(&elem_val);
        wiring.insert(&F, elem_val);

        for inp in inputs {
            // println!("> {:?} {}", inp, inp == num_2);
            if inp.len() == 5 {
                // println!("5> {:?} {:?}", inp, inp != num_2 && inp != num_3);
                if inp != num_2 && inp != num_3 {
                    num_5 = inp;
                }
            } else if inp.len() == 6 {
                if inp != num_9 {
                    if inp.contains(&wiring[&D]) {
                        num_6 = inp;
                    } else {
                        num_0 = inp;
                    }
                }
            }
        }

        let numbers = vec![
            num_0, num_1, num_2, num_3, num_4, num_5, num_6, num_7, num_8, num_9,
        ];
        // println!("{:#?}", numbers);

        let mut num_str = "".to_string();
        for output in outputs {
            // println!("{:?} {:?}", output, num_5);
            let s = numbers
                .iter()
                .enumerate()
                .find_map(|(idx, &v)| {
                    if output == v {
                        Some(idx.to_string())
                    } else {
                        None
                    }
                })
                .unwrap();

            num_str = format!("{}{}", num_str, s);
        }

        total += num_str.parse::<usize>().unwrap();

        // println!("{}", num_str);
        // break;
    }

    println!("{}", total);
}

fn update_wiring<'a>(
    wiring: &mut HashMap<&'a String, String>,
    wiring_remain: &mut HashSet<String>,
    key: &'a String,
    elems: HashSet<&String>,
) {
    assert!(elems.len() == 1);
    let elem_val = elems.iter().next().unwrap().to_string();
    wiring_remain.remove(&elem_val);
    wiring.insert(key, elem_val);
}
struct Segment7 {
    wiring: HashMap<String, String>,
}

impl Segment7 {
    fn map(self, input: String, num: usize) {}
}
