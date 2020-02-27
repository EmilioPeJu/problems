use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("Fail to read");
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    node: u32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: u32 = scan.next();
    let m: u32 = scan.next();
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let mut distance: Vec<u64> = vec![std::u64::MAX; (n + 1) as usize];
    let mut graph: Vec<Vec<(u32, u32)>> = (0..=n).map(|_| vec![]).collect();
    let mut parent: Vec<u32> = vec![0; (n + 1) as usize];
    for _ in 0..m {
        let a: u32 = scan.next();
        let b: u32 = scan.next();
        let w: u32 = scan.next();
        graph[a as usize].push((b, w));
        graph[b as usize].push((a, w));
    }
    frontier.push(State { cost: 0, node: 1 });
    distance[1] = 0;
    while let Some(State { cost, node }) = frontier.pop() {
        if node == n {
            break;
        }
        // this is the same as checking if it was visited
        if cost > distance[node as usize] {
            continue;
        }
        for &nei in graph[node as usize].iter() {
            let new_cost = cost + nei.1 as u64;
            if new_cost < distance[nei.0 as usize] {
                distance[nei.0 as usize] = new_cost;
                parent[nei.0 as usize] = node;
                frontier.push(State {
                    cost: new_cost,
                    node: nei.0,
                });
            }
        }
    }
    if distance[n as usize] == std::u64::MAX {
        println!("-1");
    } else {
        let mut current = n;
        let mut result: Vec<u32> = Vec::new();
        while current != 1 {
            result.push(current);
            current = parent[current as usize];
        }
        result.push(1);
        result.reverse();
        for &item in result.iter() {
            print!("{} ", item);
        }
        println!();
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
