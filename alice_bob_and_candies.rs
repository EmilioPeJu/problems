use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let arr: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut index = [0isize, n as isize - 1];
    let mut score = [0usize; 2];
    let mut steps = 1;
    let mut turn = 0;
    let mut acc = 0;
    let mut last_acc = 0;
    while index[0] < n as isize && index[0] <= index[1] {
        let other = 1 - turn;
        score[turn] += arr[index[turn] as usize];
        acc += arr[index[turn] as usize];
        index[turn] += if turn == 0 { 1 } else { -1 };
        if acc > last_acc {
            last_acc = acc;
            acc = 0;
            turn = other;
            if index[0] <= index[1] {
                steps += 1;
            }
        }
    }
    println!("{} {} {}", steps, score[0], score[1]);
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
