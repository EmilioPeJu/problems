use std::collections::VecDeque;
use std::io;

fn find_path(
    tree: &Vec<(usize, usize)>,
    labels: &Vec<char>,
    node: usize,
    acc: usize,
    res: &mut usize,
) {
    if tree[node] == (0, 0) {
        *res = (*res).min(acc);
        return;
    }
    // prune paths leading to worse solutions
    if acc >= *res {
        return;
    }
    if tree[node].0 > 0 {
        let mut new_acc = acc;
        if labels[node] != 'L' {
            new_acc += 1;
        }
        find_path(tree, labels, tree[node].0, new_acc, res);
    }
    if tree[node].1 > 0 {
        let mut new_acc = acc;
        if labels[node] != 'R' {
            new_acc += 1;
        }
        find_path(tree, labels, tree[node].1, new_acc, res);
    }
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let nodes: Vec<char> = scan.next::<String>().chars().collect();
    let mut tree: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        let mut l: usize = scan.next();
        if l > 0 {
            l -= 1;
        }
        let mut r: usize = scan.next();
        if r > 0 {
            r -= 1;
        }
        tree.push((l, r));
    }
    let mut result: usize = std::usize::MAX;
    find_path(&tree, &nodes, 0, 0, &mut result);
    println!("{}", result);
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
