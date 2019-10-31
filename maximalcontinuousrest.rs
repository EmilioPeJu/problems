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
    let mut arr = vec![0usize; n];
    for i in 0..n {
        let current: usize = scan.next();
        if current == 1 {
            arr[i] = 1;
        }
    }
    let mut result = 0usize;
    let mut count = 0usize;
    for i in 0..2 * n {
        // like having 2 days concatenated
        count += arr[i % n];
        if arr[i % n] == 0 {
            count = 0;
        }
        result = result.max(count);
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
