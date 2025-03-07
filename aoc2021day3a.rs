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

    fn next<T: std::str::FromStr>(&mut self) -> io::Result<T> {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break match token.parse::<T>() {
                    Ok(x) => Ok(x),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "parsing is fucked")),
                };
            }
            let line = self.next_line()?;
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn _main() -> io::Result<()> {
    let mut scan = Scan::new();
    let bits = 12usize;
    let mut counts = vec![0isize; bits];
    while let Ok(line) = scan.next::<String>() {
        for (i, c) in line.chars().enumerate() {
            counts[i] += if c == '1' { 1 } else { -1 };
        }
    }
    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    for (i, &n) in counts.iter().enumerate() {
        let bit_val = 2usize.pow((bits - i - 1) as u32);
        gamma += if n > 0 { bit_val } else { 0 };
        epsilon += if n < 0 { bit_val } else { 0 };
    }
    println!("{}", gamma * epsilon);
    Ok(())
}

fn main() -> io::Result<()> {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap()?;
    Ok(())
}
