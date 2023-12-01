use std::collections::VecDeque;
use std::io;

fn is_valid(a: &Vec<usize>, table: &Vec<Vec<usize>>) -> bool {
    let n = table.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if (a[i] | a[j]) != table[i][j] {
                return false;
            }
        }
    }
    true
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let table: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..n).map(|_| scan.next()).collect())
        .collect();
    let mut res: Vec<usize> = vec![];
    for i in 0..n {
        let mut acc = 1073741823;
        for j in 0..n {
            if i != j {
                acc = acc & table[i][j];
            }
        }
        res.push(acc);
    }
    if is_valid(&res, &table) {
        println!("YES");
        println!(
            "{}",
            res.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else {
        println!("NO");
    }
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
