use std::collections::VecDeque;
use std::io;

fn next_seq(scan: &mut Scan) -> Vec<isize> {
    return scan
        .next::<String>()
        .chars()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect();
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let a: Vec<isize> = next_seq(scan);
    let b: Vec<isize> = next_seq(scan);
    let mut balance: isize = a
        .iter()
        .fold(0isize, |acc, &c| acc + (if c == 1 { 1 } else { -1 }));
    let mut toggle = 0;
    for i in (0..n).rev() {
        if a[i] ^ toggle != b[i] {
            if balance != 0 {
                println!("NO");
                return Ok(());
            } else {
                toggle = toggle ^ 1;
            }
        }
        balance += if a[i] == 1 { -1 } else { 1 };
    }
    println!("YES");
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
