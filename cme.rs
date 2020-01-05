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
    let q: usize = scan.next();
    for _ in 0..q {
        let n: usize = scan.next();
        // x + y = z
        // x + y + z = n -> 2x + 2y = n -> x = (n - 2y) / 2
        // this implies n should be even.
        // the lowest possible is 1 + 1 = 2 -> n=4
        if n <= 4 {
            println!("{}", 4 - n);
        } else {
            println!("{}", n % 2);
        }
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
