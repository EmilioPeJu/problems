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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let k: usize = scan.next();
    let input: String = scan.next_line().chars().take(n).collect();
    let x: std::collections::HashSet<char> = scan.next_n::<char>(k).into_iter().collect();
    let mut result = 0u64;
    let mut sublen = 0u64;
    for c in input.chars() {
        if x.contains(&c) {
            sublen += 1;
        } else {
            result += sublen * (sublen + 1) / 2;
            sublen = 0;
        }
    }
    result += sublen * (sublen + 1) / 2;
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
