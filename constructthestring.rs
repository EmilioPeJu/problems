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

fn _main() {
    let mut scan = Scan::new();
    let t: usize = scan.next();
    for _ in 0..t {
        let n: usize = scan.next();
        let a: usize = scan.next();
        let b: usize = scan.next();
        let mut part: String = String::new();
        if a > b {
            part.push_str(&"a".repeat(a - b));
        }
        let mut count = 0usize;
        for c in (b'a'..=b'z').map(char::from) {
            if count == b {
                break;
            }
            part.push(c);
            count += 1;
        }
        let mut result = part.repeat(n / a);
        result.push_str(&part[0..(n % a)]);
        println!("{}", result);
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
