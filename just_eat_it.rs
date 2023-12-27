use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let arr: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    let yasser_value: i64 = arr.iter().sum();
    let mut acc = 0i64;
    let mut max = 0i64;
    let mut left_i = 0usize;
    let mut right_i = 0usize;
    for (i, &val) in arr.iter().enumerate() {
        if acc + val > 0 {
            acc += val;
        } else {
            left_i = i;
            acc = 0i64.max(val);
        }
        if acc > max {
            max = acc;
            right_i = i;
        }
    }
    if left_i == 0 && right_i == arr.len() - 1 && max == yasser_value {
        println!("YES");
    } else {
        println!("NO");
    }
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
