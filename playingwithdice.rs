struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("Fail to read");
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let a: usize = scan.next();
    let b: usize = scan.next();
    if a == b {
        println!("0 6 0");
    } else if a < b {
        // maybe it's cleaner iterating from 1 to 6 and
        // incrementing 3 counters
        let middle = a.max(b) - a.min(b) - 1;
        println!(
            "{} {} {}",
            a + middle / 2,
            middle % 2,
            6 - b + 1 + middle / 2
        );
    } else {
        let middle = a.max(b) - a.min(b) - 1;
        println!(
            "{} {} {}",
            6 - a + 1 + middle / 2,
            middle % 2,
            b + middle / 2
        );
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
