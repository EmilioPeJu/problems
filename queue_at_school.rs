use std::collections::VecDeque;
use std::io;

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
    let n: usize = scan.next();
    let t: usize = scan.next();
    let mut chars: Vec<char> = scan.next_line()?.chars().take(n).collect();
    for _t in 0..t {
        let mut i = 0;
        while i < n - 1 {
            if chars[i + 1] == 'G' && chars[i] == 'B' {
                (chars[i + 1], chars[i]) = (chars[i], chars[i + 1]);
                i += 1;
            }
            i += 1;
        }
    }
    println!("{}", chars.iter().collect::<String>());
    Ok(())
}
