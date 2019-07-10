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
    let n: usize = scan.next();
    let m: usize = scan.next();
    if n == m {
        println!("0");
    } else if m % n != 0 {
        println!("-1");
    } else {
        let mut rem = m / n;
        let mut steps = 0usize;
        while rem % 2 == 0 {
            steps += 1;
            rem /= 2;
        }
        while rem % 3 == 0 {
            steps += 1;
            rem /= 3;
        }
        if rem == 1 {
            println!("{}", steps);
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
