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

fn apply_step(arr: &[f64; 21], c: char) -> [f64; 21] {
    let mut arr2 = [0f64; 21];
    for i in 0..=20 {
        if c == '+' && i < 20 {
            arr2[i + 1] = arr[i];
        }
        if c == '-' && i > 0 {
            arr2[i - 1] = arr[i];
        }
        if c == '?' {
            if i < 20 {
                arr2[i + 1] += arr[i] * 0.5;
            }
            if i > 0 {
                arr2[i - 1] += arr[i] * 0.5;
            }
        }
    }
    arr2
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let mut arr = [0f64; 21];
    // positions offset by 10 to avoid negative indexes
    arr[10] = 1.0;
    let target: isize = scan
        .next::<String>()
        .chars()
        .map(|x| if x == '+' { 1 } else { -1 })
        .sum();
    for c in scan.next::<String>().chars() {
        let arr2 = apply_step(&arr, c);
        arr = arr2;
    }
    println!("{:.12}", arr[(target + 10) as usize]);
    Ok(())
}
