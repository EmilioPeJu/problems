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

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let k: usize = scan.next();
    let sd: Vec<usize> = scan.next_n(n);
    let mut count = [0usize; 3001];
    let mut even = 0usize;
    let mut odds = 0usize;
    for &item in &sd {
        count[item] += 1;
    }
    for i in 1..=k {
        even += count[i] / 2;
        odds += count[i] % 2;
    }
    let sets_left = n / 2 + n % 2 - even;
    let result = even * 2 + odds.min(sets_left);
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
