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

fn is_good(x: usize) -> bool {
    let mut isused: [bool; 10] = [false; 10];
    let mut a = x;
    while a > 0 {
        let dig = a % 10;
        a /= 10;
        if isused[dig] {
            return false;
        }
        isused[dig] = true;
    }
    return true;
}

fn _main() {
    let mut scan = Scan::new();
    let l: usize = scan.next();
    let r: usize = scan.next();
    for i in l..=r {
        if is_good(i) {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
