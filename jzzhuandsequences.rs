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
}

static BIG: i64 = 1e9 as i64 + 7;
fn _main() {
    let mut scan = Scan::new();
    let mut x: i64 = scan.next();
    let mut y: i64 = scan.next();
    let mut n: i64 = scan.next();
    let mut i = 2i64;
    n = n % 6;
    // I think it would be better to precompute an
    // array with the 6 different possible values
    while i < n + 6 {
        let tmp = y;
        y = (y - x + BIG) % BIG;
        x = tmp;
        i += 1;
    }
    println!("{}", (y + BIG) % BIG);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
