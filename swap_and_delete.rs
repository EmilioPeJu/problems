use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let input: Vec<u8> = scan
        .next::<String>()
        .chars()
        .map(|c| if c == '1' { 1u8 } else { 0u8 })
        .collect();
    let (mut n_zeros, mut n_ones) = input.iter().fold((0usize, 0usize), |acc, &el| {
        if el == 0 {
            (acc.0 + 1, acc.1)
        } else {
            (acc.0, acc.1 + 1)
        }
    });
    let mut i = 0usize;
    while i < input.len() {
        if input[i] == 0 {
            if n_ones == 0 {
                break;
            }
            n_ones -= 1;
        } else {
            if n_zeros == 0 {
                break;
            }
            n_zeros -= 1;
        }
        i += 1;
    }

    let res = input.len() - i;
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
