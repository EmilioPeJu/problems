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
        let mut n: u64 = scan.next();
        let mut counts = 0usize;
        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
                counts += 1;
            } else if n % 3 == 0 {
                n /= 3;
                counts += 2;
            } else if n % 5 == 0 {
                n /= 5;
                counts += 3;
            } else {
                break;
            }
        }
        if n == 1 {
            println!("{}", counts);
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
