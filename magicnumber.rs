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
    let mut input: usize = scan.next();
    let mut result = true;
    let mut count4 = 0usize;
    let mut digit = 0usize;
    while input > 0 {
        digit = input % 10;
        input /= 10;
        if digit == 4 {
            count4 += 1;
            if count4 > 2 {
                result = false;
                break;
            }
        } else if digit == 1 {
            count4 = 0;
        } else {
            result = false;
            break;
        }
    }
    println!("{}", if result && digit != 4 { "YES" } else { "NO" });
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
