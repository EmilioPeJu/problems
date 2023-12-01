use std::cmp::Ordering;
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
                    _ => Err(io::Error::new(io::ErrorKind::Other, "parsing problem")),
                };
            }
            let line = self.next_line()?;
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

#[derive(Eq, PartialEq, Ord, Default, Clone)]
struct Friend {
    eat: isize,
    money: isize,
}

impl PartialOrd for Friend {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.money - self.eat).partial_cmp(&(other.money - other.eat))
    }
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next().unwrap();
    for _ in 0..n {
        let t: usize = scan.next().unwrap();
        let mut together: Vec<Friend> = vec![Friend::default(); t];
        for i in 0..t {
            together[i].eat = scan.next().unwrap();
        }
        for i in 0..t {
            together[i].money = scan.next().unwrap();
        }
        together.sort();
        let mut res = 0u64;
        let mut m1 = 0usize;
        let mut m2 = t - 1;
        while m1 < m2 {
            if together[m1].eat + together[m2].eat <= together[m1].money + together[m2].money {
                res += 1;
                m1 += 1;
                m2 -= 1;
            } else {
                m1 += 1;
            }
        }
        println!("{}", res);
    }
    Ok(())
}
