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
    // if m>2*n, then we can take m teams
    // if n>2*m, we can take n teams
    // otherwise some type1 (a) and some type2 (b)
    // a+2b = n; 2a+b = m
    // a=n-2b, 2n-4b+b=m -> b=(2n-m)/3
    // b=m-2a, a+2m-4a=n -> a=(2m-n)/3
    // a+b = (n+m)/3
    let result = n.min(m).min((n + m) / 3);
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
