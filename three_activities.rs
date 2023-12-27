use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let b: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let c: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut sa: Vec<(usize, usize)> = a.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    let mut sb: Vec<(usize, usize)> = b.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    let mut sc: Vec<(usize, usize)> = c.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    sa.sort_by_key(|x| x.1);
    sa.reverse();
    sb.sort_by_key(|x| x.1);
    sb.reverse();
    sc.sort_by_key(|x| x.1);
    sc.reverse();
    let mut res = 0usize;
    for i in 0..3.min(n) {
        for j in 0..3.min(n) {
            for k in 0..3.min(n) {
                if sa[i].0 != sb[j].0 && sa[i].0 != sc[k].0 && sb[j].0 != sc[k].0 {
                    res = res.max(sa[i].1 + sb[j].1 + sc[k].1);
                }
            }
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
