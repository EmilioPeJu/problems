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
    let mut cnt = [0usize; 1024];
    let n: usize = scan.next();
    let mut maxval = 0usize;
    let mut ndiff = 0usize;
    for _ in 0..n {
        let current: usize = scan.next();
        if cnt[current] == 0 {
            ndiff += 1;
        }
        cnt[current] += 1;
        maxval = maxval.max(cnt[current]);
    }
    println!("{} {}", maxval, ndiff);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
