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
    let mut evensum: u64 = 0;
    let n: usize = scan.next();
    let mut odds: Vec<u64> = vec![];

    for _ in 0..n {
        let current: u64 = scan.next();
        if current % 2 == 0 {
            evensum += current;
        } else {
            odds.push(current);
        }
    }
    odds.sort();
    odds.reverse();
    let oddsum: u64 = odds.iter().take(odds.len() - odds.len() % 2).sum();
    println!("{}", oddsum + evensum);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
