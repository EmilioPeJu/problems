use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let mut arr: Vec<u8> = scan
        .next::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let n = arr.len();
    let mut candidates: Vec<usize> = vec![];
    for i in 0..(n - 1) {
        if arr[i] % 2 == 0 {
            candidates.push(i);
            if arr[i] < arr[n - 1] {
                break;
            }
        }
    }
    if candidates.len() == 0 {
        println!("-1");
    } else {
        let &last_i = candidates.last().unwrap();
        (arr[n - 1], arr[last_i]) = (arr[last_i], arr[n - 1]);
        for x in arr {
            print!("{}", x);
        }
        println!();
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
