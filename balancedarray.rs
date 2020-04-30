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
    let t: usize = scan.next();
    for _ in 0..t {
        let n: usize = scan.next();
        if (n / 2 - 1) % 2 == 0 {
            println!("NO");
            continue;
        }
        println!("YES");
        let mut sum = 0u64;
        let mut current = 2usize;
        for _ in 0..(n / 2) {
            print!("{} ", current);
            sum += current as u64;
            current += 2;
        }
        current = 1;
        for _ in 0..(n / 2 - 1) {
            print!("{} ", current);
            sum -= current as u64;
            current += 2;
        }
        println!("{}", sum);
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
