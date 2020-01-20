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
    let t: usize = scan.next();
    for _ in 0..t {
        let a: usize = scan.next();
        let b: usize = scan.next();
        let c: usize = scan.next();
        let d: usize = scan.next();
        let k: usize = scan.next();
        let npen = a / c + if a % c != 0 { 1 } else { 0 };
        let npencil = b / d + if b % d != 0 { 1 } else { 0 };
        if npen + npencil <= k {
            println!("{} {}", npen, npencil);
        } else {
            println!("-1");
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
