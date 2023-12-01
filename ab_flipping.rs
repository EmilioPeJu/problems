use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let mut input: Vec<char> = scan.next::<String>().chars().collect();
    let mut res = 0usize;
    let mut block: Vec<bool> = vec![false; n];
    for i in (0..=(n - 2)).rev() {
        if input[i] == 'B' || input[i + 1] == 'A' {
            continue;
        }
        block[i] = true;
        let mut j = i + 1;
        while j + 1 < n && !block[j] && input[j + 1] == 'B' {
            block[j] = true;
            j += 1;
        }
        res += j - i;
        (input[i], input[j]) = (input[j], input[i]);
    }
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
