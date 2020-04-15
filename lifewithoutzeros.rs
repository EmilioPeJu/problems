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

fn no_zeros(x: u64) -> u64 {
    let mut result = 0u64;
    let mut current = x;
    let mut current10 = 1;
    while current > 0 {
        let dig = current % 10;
        if dig != 0 {
            result += current10 * dig;
            current10 *= 10;
        }
        current /= 10;
    }
    result
}

fn _main() {
    let mut scan = Scan::new();
    let a: u64 = scan.next();
    let b: u64 = scan.next();
    let c = a + b;
    println!(
        "{}",
        if no_zeros(a) + no_zeros(b) == no_zeros(c) {
            "YES"
        } else {
            "NO"
        },
    );
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
