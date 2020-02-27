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
    let mut n0: usize = scan.next();
    let mut n1: usize = scan.next();
    if !(n1 <= 2 * n0 + 2) || !(n1 >= n0 - 1) {
        println!("-1");
        return;
    }
    let mut group2 = 0usize;
    if n0 > 0 && n1 > n0 {
        group2 = (n1 - n0).min(n0);
        n0 -= group2;
        n1 -= 2 * group2;
    }
    let mut group1 = 0usize;
    if n0 > 0 && n1 > 0 {
        group1 = n1.min(n0);
        n0 -= group1;
        n1 -= group1;
    }
    if n0 != 0 {
        print!("0");
    }
    print!("{}", "110".repeat(group2));
    print!("{}", "10".repeat(group1));
    if n1 != 0 {
        print!("{}", "1".repeat(n1));
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
