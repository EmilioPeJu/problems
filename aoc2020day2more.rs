use std::collections::VecDeque;
use std::io;

struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
        }
    }

    fn next_line(&self) -> io::Result<String> {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line)? {
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
    let mut result = 0usize;
    loop {
        let policy: String = match scan.next() {
            Ok(x) => x,
            _ => break,
        };
        let separator = policy.find('-').unwrap();
        let pol1: isize = (&policy[0..separator]).parse::<isize>().unwrap() - 1isize;
        let pol2: isize = (&policy[(separator + 1)..]).parse::<isize>().unwrap() - 1isize;
        let _letter: String = scan.next().unwrap();
        let letter: char = _letter.chars().next().unwrap();
        let _pass: String = scan.next().unwrap();
        let pass: Vec<char> = _pass.chars().collect();
        let pol1valid: bool =
            (pol1 as usize) < pass.len() && pol1 >= 0 && pass[pol1 as usize] == letter;
        let pol2valid: bool =
            (pol2 as usize) < pass.len() && pol2 >= 0 && pass[pol2 as usize] == letter;
        if pol1valid ^ pol2valid {
            result += 1;
        }
    }
    println!("{}", result);
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
