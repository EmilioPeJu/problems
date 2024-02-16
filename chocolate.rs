use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut got_one = false;
    let mut zero_count = 1usize;
    let mut result = 1usize;
    for _ in 0..n {
        let ai: u8 = scan.next();
        if ai == 1 {
            got_one = true;
            result *= zero_count;
            zero_count = 1;
        }

        if got_one && ai == 0 {
            zero_count += 1;
        }
    }

    if !got_one {
        println!("0");
    } else {
        println!("{}", result);
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
