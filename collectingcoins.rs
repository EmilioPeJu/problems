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
    let ts: usize = scan.next();
    for _ in 0..ts {
        let a: usize = scan.next();
        let b: usize = scan.next();
        let c: usize = scan.next();
        let n: usize = scan.next();
        let biggest = a.max(b).max(c);
        let min_required = 3 * biggest - a - b - c;
        let result = n >= min_required && (n - min_required) % 3 == 0;
        println!("{}", if result { "YES" } else { "NO" });
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
