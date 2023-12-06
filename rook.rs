use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let line: Vec<char> = scan.next::<String>().chars().collect();
    let c = line[0];
    let r = line[1].to_digit(10).unwrap();
    for i in 1..=8 {
        if i != r {
            println!("{}{}", c, i);
        }
    }
    for j in 'a'..='h' {
        if c != j {
            println!("{}{}", j, r);
        }
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
