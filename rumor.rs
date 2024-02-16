use std::collections::VecDeque;
use std::io;

type Graph = Vec<Vec<usize>>;

fn dfs(graph: &Graph, visited: &mut Vec<bool>, current: usize) {
    visited[current] = true;
    for &nei in &graph[current] {
        if !visited[nei] {
            dfs(graph, visited, nei);
        }
    }
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let c: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut graph: Graph = vec![vec![]; n];
    for _ in 0..m {
        let x: usize = scan.next();
        let y: usize = scan.next();
        graph[x - 1].push(y - 1);
        graph[y - 1].push(x - 1);
    }
    let mut sorted_inds: Vec<usize> = (0..n).collect();
    sorted_inds.sort_by_key(|&i| c[i]);
    let mut visited = vec![false; n];
    let mut res = 0u64;
    for i in 0..n {
        let j = sorted_inds[i];
        if !visited[j] {
            res += c[j] as u64;
            dfs(&graph, &mut visited, j);
        }
    }
    println!("{}", res);
    Ok(())
}

struct Scan {
    buffer: VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
        }
    }

    fn next_line(&self) -> io::Result<String> {
        let mut line = String::new();
        match io::stdin().read_line(&mut line)? {
            0 => Err(io::Error::new(io::ErrorKind::Other, "EOF")),
            _ => Ok(line),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                match token.parse() {
                    Ok(x) => {
                        return x;
                    }
                    _ => {
                        panic!("parse");
                    }
                }
            }
            let line = self.next_line().unwrap();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}
