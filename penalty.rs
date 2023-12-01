use std::collections::VecDeque;
use std::io;

fn least_time(input: &Vec<char>) -> usize {
    let mut points = [0usize; 2];
    let mut left: [usize; 2] = [5, 5];
    for i in 0..10 {
        points[i % 2] += if input[i] == '1' { 1 } else { 0 };
        left[i % 2] -= 1;
        if points[0] > points[1] + left[1] || points[1] > points[0] + left[0] {
            return i + 1;
        }
    }
    return 10;
}

fn solve(scan: &mut Scan) -> io::Result<()> {
    let input: Vec<char> = scan.next_line().unwrap().chars().collect();
    let mut res = std::usize::MAX;
    for team in 0..=1 {
        let favoured_team: Vec<char> = input
            .iter()
            .enumerate()
            .map(|(i, &val)| match val {
                '?' if i % 2 == team => '1',
                '?' if i % 2 == 1 - team => '0',
                _ => val,
            })
            .collect();
        let time = least_time(&favoured_team);
        res = res.min(time);
    }
    println!("{}", res);
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

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        solve(&mut scan)?;
    }
    Ok(())
}
