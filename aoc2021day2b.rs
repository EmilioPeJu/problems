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
                    _ => Err(io::Error::new(io::ErrorKind::Other, "parsing is fucked")),
                };
            }
            let line = self.next_line()?;
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn _main() -> io::Result<()> {
    let mut scan = Scan::new();
    let mut aim = 0isize;
    let mut x = 0isize;
    let mut y = 0isize;
    loop {
        let command = match scan.next::<String>() {
            Ok(val) => val,
            Err(_) => break,
        };
        match &command[..] {
            "forward" => {
                let val = scan.next::<isize>().unwrap();
                x += val;
                y += val * aim;
            }
            "down" => aim += scan.next::<isize>().unwrap(),
            "up" => aim -= scan.next::<isize>().unwrap(),
            _ => (),
        }
    }
    println!("{}", x * y);
    Ok(())
}

fn main() -> io::Result<()> {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap()?;
    Ok(())
}
