use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

fn expand(t: (i64, i64), a: i64, b: i64) -> HashSet<(i64, i64)> {
    let mut res: HashSet<(i64, i64)> = HashSet::new();
    for (step_x, step_y) in [
        (a, b),
        (a, -b),
        (-a, b),
        (-a, -b),
        (b, a),
        (b, -a),
        (-b, a),
        (-b, -a),
    ] {
        res.insert((t.0 + step_x, t.1 + step_y));
    }
    res
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let a: i64 = scan.next();
    let b: i64 = scan.next();
    let pk: (i64, i64) = (scan.next(), scan.next());
    let pq: (i64, i64) = (scan.next(), scan.next());
    let mut res = 0usize;
    for (pos_x, pos_y) in expand(pk, a, b) {
        let diff_x = (pos_x - pq.0).abs();
        let diff_y = (pos_y - pq.1).abs();
        if diff_x == a && diff_y == b || diff_x == b && diff_y == a {
            res += 1;
        }
    }
    println!("{}", res);
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
