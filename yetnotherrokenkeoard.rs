use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let mut index1: Vec<usize> = vec![];
    let mut index2: Vec<usize> = vec![];
    for (i, &c) in input.iter().enumerate() {
        if c == 'b' {
            index1.pop();
        } else if c == 'B' {
            index2.pop();
        } else {
            if c.is_lowercase() {
                index1.push(i);
            } else if c.is_uppercase() {
                index2.push(i);
            }
        }
    }
    index1.extend(&index2);
    index1.sort();
    let output: String = index1.iter().map(|&i| input[i]).collect();
    println!("{}", output);
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
