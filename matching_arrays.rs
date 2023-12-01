use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let x: usize = scan.next();
    let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut b: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut a_index: Vec<usize> = (0..n).collect();
    a_index.sort_by_key(|&i| a[i]);
    b.sort();
    b.rotate_left(x);
    let mut b2 = vec![0usize; n];
    for i in 0..n {
        b2[a_index[i]] = b[i];
    }
    let beauty: usize = (0..n).map(|i| if a[i] > b2[i] { 1 } else { 0 }).sum();
    if beauty == x {
        println!("YES");
        println!(
            "{}",
            b2.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else {
        println!("NO");
    }
    Ok(())
}

fn _main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        solve1(&mut scan)?;
    }
    Ok(())
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
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
