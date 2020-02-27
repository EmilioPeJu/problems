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
    // r(a_1 a_2 a_3...a_n) = a_2 a_3 .. a_n+1
    // -
    // a_1 a_2 a_3..a_n
    // ----------------
    // rS - S
    // therefore
    // S(r-1)=a_n+1 -a_1
    // S = (a_n+1 - a_1) / (r - 1)
    let result: u64 = 2u64.pow(n as u32 + 1) - 2;
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
