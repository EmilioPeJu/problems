use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let _n: usize = scan.next();
    let k: usize = scan.next();
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let mut print_needed = vec![true; input.len()];
    let mut removed = 0usize;
    for c1 in 'a'..='z' {
        for (i, &c2) in input.iter().enumerate() {
            if c1 == c2 && removed < k {
                removed += 1;
                print_needed[i] = false;
            }
            if removed >= k {
                break;
            }
        }
    }
    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if print_needed[i] { Some(c) } else { None })
            .collect::<String>()
    );
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
