use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let _n: usize = scan.next();
    let mut input: Vec<char> = scan.next::<String>().chars().collect();
    let res: usize = loop {
        let mut new_input: Vec<char> = vec![];
        let mut i = 0;
        while i < input.len() {
            if i + 1 == input.len() {
                new_input.push(input[i]);
                i += 1;
            } else if input[i] == input[i + 1] {
                new_input.push(input[i]);
                i += 1;
            } else {
                i += 2;
            }
        }
        if input.len() == new_input.len() {
            break input.len();
        }
        input = new_input;
    };
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
