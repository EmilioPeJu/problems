use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let _n: usize = scan.next();
    let k: usize = scan.next();
    let arr: Vec<u8> = scan
        .next::<String>()
        .chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect();
    let mut res = 0usize;
    let mut i = 0usize;
    let mut first = true;
    while i < arr.len() {
        let mut acc = 0usize;
        while i < arr.len() && arr[i] == 0 {
            i += 1;
            acc += 1;
        }
        if first {
            acc += k;
        }
        if i == arr.len() && arr[i - 1] == 0 {
            acc += k;
        }
        if acc >= k + 1 {
            res += (acc + 1) / (k + 1) - 1;
        }
        first = false;
        i += 1;
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
