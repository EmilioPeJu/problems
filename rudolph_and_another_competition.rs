use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::VecDeque;
use std::io;

#[derive(Eq, PartialEq, Ord, Debug)]
struct Participant {
    problems: usize,
    penalty: u64,
    i: isize,
}

impl PartialOrd for Participant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.problems != other.problems {
            return self.problems.partial_cmp(&other.problems);
        } else if self.penalty != other.penalty {
            return other.penalty.partial_cmp(&self.penalty);
        } else {
            return other.i.partial_cmp(&self.i);
        }
    }
}

fn solve(scan: &mut Scan) -> io::Result<()> {
    let n: isize = scan.next();
    let m: isize = scan.next();
    let h: u64 = scan.next();
    let mut participants: Vec<Participant> = vec![];
    for i in 0..n {
        let mut arr: Vec<u64> = (0..m).map(|_| scan.next()).collect();
        arr.sort();
        let mut t_acc = 0u64;
        let mut penalty = 0u64;
        let mut problems = 0usize;
        for &val in arr.iter() {
            t_acc += val;
            if t_acc > h {
                break;
            }
            problems += 1;
            penalty += t_acc;
        }
        participants.push(Participant {
            problems,
            penalty,
            i,
        });
    }
    participants.sort();
    participants.reverse();
    for (pos, participant) in participants.iter().enumerate() {
        if participant.i == 0 {
            println!("{}", pos + 1);
            break;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        solve(&mut scan)?;
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
