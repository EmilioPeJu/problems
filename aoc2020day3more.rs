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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>().unwrap()).collect()
    }
}

fn calc_slope(lines: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut result = 0usize;
    let mut row = 0usize;
    let mut col = 0usize;
    let nrow = lines.len();
    let ncol = lines[0].len();
    while row < nrow {
        if lines[row][col] == '#' {
            result += 1;
        }
        row += down;
        col = (col + right) % ncol;
    }
    result
}

fn _main() -> io::Result<()> {
    let mut scan = Scan::new();
    let mut lines: Vec<Vec<char>> = vec![];
    loop {
        let line: Vec<char> = match scan.next::<String>() {
            Ok(x) => x,
            _ => break,
        }
        .chars()
        .collect();
        lines.push(line);
    }
    let mut result = 1usize;
    for &(right, down) in &[(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let current = calc_slope(&lines, right, down);
        result *= current;
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
