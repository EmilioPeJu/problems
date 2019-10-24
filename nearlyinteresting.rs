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

fn is_interesting(x: usize) -> bool {
    let mut aux = x;
    let mut dsum = 0usize;
    while aux > 0 {
        dsum += aux % 10;
        aux /= 10;
    }
    dsum % 4 == 0
}

fn _main() {
    let mut scan = Scan::new();
    let mut a: usize = scan.next();
    while !is_interesting(a) {
        a += 1;
    }
    println!("{}", a);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
