use std::collections::VecDeque;
use std::io;

type Graph = Vec<Vec<usize>>;

fn _count_branches(graph: &Graph, current: usize, visited: &mut Vec<bool>) -> usize {
    if visited[current] {
        return 0;
    }
    visited[current] = true;
    let mut res = 0usize;
    for &nei in &graph[current] {
        if !visited[nei] {
            res += _count_branches(graph, nei, visited);
        }
    }
    if res == 0 {
        return 1;
    } else {
        return res;
    }
}

fn count_branches(graph: &Graph, start: usize) -> usize {
    let mut visited: Vec<bool> = vec![false; graph.len() + 1];
    return _count_branches(graph, start, &mut visited);
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let mut graph: Graph = vec![vec![]; n + 1];
    let mut start = 1usize;
    for _ in 0..(n - 1) {
        let u: usize = scan.next();
        let v: usize = scan.next();
        graph[u].push(v);
        graph[v].push(u);
        start = u;
    }
    let mut result = count_branches(&graph, start);
    if graph[start].len() < 2 {
        result += 1;
    }
    println!("{}", (result + 1) / 2);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        solve1(&mut scan)?;
    }
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
