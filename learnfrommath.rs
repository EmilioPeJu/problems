struct Scan {
    buffer: std::collections::VecDeque<String>
}

impl Scan {
    fn new() -> Scan {
        Scan { buffer: std::collections::VecDeque::new() }
    }

    fn next<T: std::str::FromStr>(&mut self)-> T {
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

fn not_prime(x: usize) -> bool {
    for i in 2..x {
        if x % i == 0 { return true; }
    }
    false
}

fn main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    for i in 4..n {
        let j = n - i;
        if not_prime(i) && not_prime(j) {
            println!("{} {}", i, j);
            break;
        }
    }
}
