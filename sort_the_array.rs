use std::collections::VecDeque;
use std::io;

fn is_sorted(a: &[usize]) -> bool {
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut i1 = 0usize;
    let mut i2 = 0usize;
    let mut last_dir = 1;
    for i in 0..(n - 1) {
        let dir = a[i + 1] as isize - a[i] as isize;
        if dir < 0 && last_dir > 0 {
            i1 = i;
            i2 = n - 1;
        } else if dir > 0 && last_dir < 0 {
            i2 = i;
            break;
        }
        last_dir = dir;
    }
    a[i1..=i2].reverse();
    if is_sorted(&a) {
        println!("yes\n{} {}", i1 + 1, i2 + 1);
    } else {
        println!("no");
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
