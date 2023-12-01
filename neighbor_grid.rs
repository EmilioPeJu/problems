use std::collections::VecDeque;
use std::io;

fn needed(i: usize, j: usize, n: usize, m: usize) -> usize {
    let mut res = 4;
    if i == 0 || i == n - 1 {
        res -= 1;
    }
    if j == 0 || j == m - 1 {
        res -= 1;
    }
    res
}

fn solve(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut arr: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..m).map(|_| scan.next::<usize>()).collect())
        .collect();
    for i in 0..n {
        for j in 0..m {
            let want = needed(i, j, n, m);
            if arr[i][j] > want {
                println!("NO");
                return Ok(());
            } else {
                arr[i][j] = want;
            }
        }
    }
    println!("YES");
    for i in 0..n {
        for j in 0..m {
            print!("{} ", arr[i][j]);
        }
        println!();
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

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        solve(&mut scan)?;
    }
    Ok(())
}
