use std::collections::VecDeque;
use std::io;

fn is_valid(arr: &[[u8; 50]; 50], i: usize, j: usize) -> bool {
    if arr[i][j] == 0 {
        // || i == 49 || j == 49
        return true;
    }
    arr[i + 1][j] == 1 || arr[i][j + 1] == 1
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let mut arr = [[0u8; 50]; 50];
    for i in 0..n {
        for (j, c) in scan.next::<String>().chars().enumerate() {
            arr[i][j] = if c == '0' { 0 } else { 1 };
        }
    }
    for i in 0..(n - 1) {
        for j in 0..(n - 1) {
            if !is_valid(&arr, i, j) {
                println!("NO");
                return Ok(());
            }
        }
    }
    println!("YES");
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
