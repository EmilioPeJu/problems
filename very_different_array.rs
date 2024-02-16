use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut a: Vec<isize> = (0..n).map(|_| scan.next()).collect();
    let mut b: Vec<isize> = (0..m).map(|_| scan.next()).collect();
    a.sort();
    b.sort();
    b.reverse();
    let mut res = 0u64;
    let mut ai = 0isize;
    let mut aj = (n - 1) as isize;
    let mut bi = 0isize;
    let mut bj = (m - 1) as isize;
    while ai <= aj {
        let first = (a[ai as usize] - b[bi as usize]).abs();
        let second = (a[aj as usize] - b[bj as usize]).abs();
        if first >= second {
            res += first as u64;
            ai += 1;
            bi += 1;
        } else {
            res += second as u64;
            aj -= 1;
            bj -= 1;
        }
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
