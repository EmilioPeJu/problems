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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _gcd(x: u64, y: u64) -> u64 {
    let mut a = x.max(y);
    let mut b = x.min(y);
    while b != 0 {
        let aux = b;
        b = a % b;
        a = aux;
    }
    a
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let arr: Vec<u64> = scan.next_n(n);
    let mut all_g = arr[0];
    for item in &arr {
        all_g = _gcd(all_g, *item);
    }
    let mut count = 0usize;
    let mut i = 1u64;
    // finding number of divisors in O(sqrt(n))
    while i * i <= all_g {
        if all_g % i == 0 {
            count += 1;
            if i != all_g / i {
                count += 1;
            }
        }
        i += 1;
    }
    println!("{}", count);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
