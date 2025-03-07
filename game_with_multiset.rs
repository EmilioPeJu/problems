use std::collections::VecDeque;
use std::io;

fn can_do_it(mut counts: [usize; 30], mut num: usize) -> bool {
    let mut power = 1usize;
    for i in 0..30 {
        if (num & power) != 0 {
            if counts[i] == 0 {
                return false;
            }
            counts[i] -= 1;
            num -= power;
        }
        if i < 29 {
            counts[i + 1] += counts[i] / 2;
        }
        power = power << 1;
    }
    num == 0
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let m: usize = scan.next();
    let mut counts = [0usize; 30];
    for _ in 0..m {
        let cmd: usize = scan.next();
        let num: usize = scan.next();
        if cmd == 1 {
            counts[num] += 1;
        } else {
            let res: bool = can_do_it(counts.clone(), num);
            println!("{}", if res { "YES" } else { "NO" });
        }
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
