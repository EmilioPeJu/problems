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
    let mut arr = [[0isize; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let x: isize = scan.next();
            for (a, b) in [(-1isize, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
                let new_i = i + a;
                let new_j = j + b;
                if new_i >= 0 && new_j >= 0 && new_i < 3 && new_j < 3 {
                    arr[new_i as usize][new_j as usize] += x;
                }
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            print!("{}", if arr[i][j] % 2 == 0 { 1 } else { 0 });
        }
        println!();
    }
    Ok(())
}
