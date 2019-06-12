use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, VecDeque};
use std::option::Option;

struct Scan {
    buffer: VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
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

struct State {
    cost: usize,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // reversed to convert the max-heap in min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        other.cost == self.cost
    }
}

impl Eq for State {}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut graph = vec![vec![]; n];
    let is_cat: Vec<usize> = (0..n).map(|_| scan.next::<usize>()).collect();
    for _ in 0..(n - 1) {
        let x: usize = scan.next();
        let y: usize = scan.next();
        graph[x - 1].push(y - 1);
        graph[y - 1].push(x - 1);
    }
    let inf: usize = std::usize::MAX;
    let mut dist = vec![inf; n];
    let mut pq = BinaryHeap::new();
    let mut visited = vec![false; n];
    pq.push(State {
        cost: is_cat[0],
        node: 0,
    });
    dist[0] = is_cat[0];
    let mut is_leaf = vec![false; n];
    while let Some(current) = pq.pop() {
        if visited[current.node] || current.cost > m {
            continue;
        }
        visited[current.node] = true;
        let mut leaf = true;
        for &adj in graph[current.node].iter() {
            if visited[adj] {
                continue;
            }
            leaf = false;
            let new_dist = if is_cat[current.node] == 0 {
                0
            } else {
                current.cost
            } + is_cat[adj];
            if new_dist < dist[adj] {
                dist[adj] = new_dist;
                pq.push(State {
                    cost: new_dist,
                    node: adj,
                });
            }
        }
        is_leaf[current.node] = leaf;
    }
    let mut result = 0usize;
    for i in 0..n {
        if is_leaf[i] {
            if dist[i] <= m {
                result += 1;
            }
        }
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
