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
    let s: usize = scan.next();
    let v1: usize = scan.next();
    let v2: usize = scan.next();
    let t1: usize = scan.next();
    let t2: usize = scan.next();
    let score1 = s * v1 + 2 * t1;
    let score2 = s * v2 + 2 * t2;
    if score1 == score2 {
        println!("Friendship");
    } else {
        println!("{}", if score1 < score2 { "First" } else { "Second" });
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
