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

fn gcd(a: usize, b: usize) -> usize {
    let mut x = a;
    let mut y = b;
    let mut tmp = 0usize;
    while x > 0 {
        tmp = x;
        x = y % x;
        y = tmp;
    }
    y
}

fn _main() {
    let mut scan = Scan::new();
    let a: usize = scan.next();
    let b: usize = scan.next();
    let c: usize = scan.next();
    let d: usize = scan.next();
    let mut p = 0usize;
    let mut q = 0usize;
    if a * d >= c * b {
        p = c * b;
        q = a * d;
    } else {
        p = d * a;
        q = b * c;
    }
    p = q - p;
    let m = gcd(p, q);
    p = p / m;
    q = q / m;
    println!("{}/{}", p, q);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
