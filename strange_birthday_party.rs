use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut k: Vec<usize> = (0..n).map(|_| scan.next::<usize>() - 1).collect();
    let c: Vec<usize> = (0..m).map(|_| scan.next()).collect();
    k.sort();
    let mut ik: isize = n as isize - 1;
    let mut ic: usize = 0;
    let mut res = 0u64;
    while ic < m && ik >= 0 && k[ik as usize] >= ic {
        res += c[ic] as u64;
        ic += 1;
        ik -= 1;
    }
    for i in 0..=ik {
        res += c[k[i as usize]] as u64;
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
