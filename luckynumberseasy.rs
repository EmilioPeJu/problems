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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn find(src: u64, num: u64, n4: usize, n7: usize, result: &mut u64) {
    if num > 10000000000 {
        return;
    }
    if n4 == n7 && num >= src && (num - src) < (*result - src) {
        *result = num;
    }
    find(src, num * 10 + 4, n4 + 1, n7, result);
    find(src, num * 10 + 7, n4, n7 + 1, result);
}

fn _main() {
    let mut scan = Scan::new();
    let n: u64 = scan.next();
    let mut result = std::u64::MAX;
    find(n, 0, 0, 0, &mut result);
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
