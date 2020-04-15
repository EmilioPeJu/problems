struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next_line(&self) -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Fail to read");
        line
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let line = self.next_line();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let x: usize = scan.next();
    let y: usize = scan.next();
    let needed = y * n / 100 + if y * n % 100 == 0 { 0 } else { 1 };
    println!("{}", if needed <= x { 0 } else { needed - x });
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
