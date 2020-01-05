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

fn is_prime(x: usize) -> bool {
    for i in 2..=((x as f64).sqrt() as usize) {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn next_prime(x: usize) -> usize {
    let mut aux = x + 1;
    while !is_prime(aux) {
        aux += 1;
    }
    aux
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let result = next_prime(n) == m;
    println!("{}", if result { "YES" } else { "NO" });
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
