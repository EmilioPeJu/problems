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
    let mut a = scan.next::<usize>();
    let mut b = scan.next::<usize>();
    let mut c = scan.next::<usize>();
    let mut d = scan.next::<usize>();
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if a < c {
        std::mem::swap(&mut a, &mut c);
    }
    if a < d {
        std::mem::swap(&mut a, &mut d);
    }
    println!("{} {} {}", b + c - a, b + d - a, c + d - a);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
