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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>().unwrap()).collect()
    }
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next().unwrap();
    for _ in 0..ts {
        let n: usize = scan.next().unwrap();
        let a: Vec<usize> = scan.next_n(n);
        let all_ones = a[1..n - 1].iter().all(|&x| x <= 1);
        if n == 3 && a[1] % 2 == 1 || all_ones {
            println!("-1");
        } else {
            let res: usize = a[1..n - 1].iter().map(|x| (x + 1) / 2).sum();
            println!("{}", res);
        }
    }
    Ok(())
}
