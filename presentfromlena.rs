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

fn printseq(x: usize, n: usize) {
    let spaces = "  ".repeat(n - x);
    print!("{}", spaces);
    for i in 0..x {
        print!("{} ", i);
    }
    print!("{}", x);
    for i in (0..x).rev() {
        print!(" {}", i);
    }
    println!();
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    for i in 0..n {
        printseq(i, n);
    }
    printseq(n, n);
    for i in (0..n).rev() {
        printseq(i, n);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
