use std::collections::HashMap;

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
    let s: String = scan.next();
    let mut count: HashMap<char, usize> = HashMap::new();
    for ch in s.chars() {
        count.entry(ch).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut odds = 0usize;
    for (_k, v) in count.iter() {
        odds += v % 2;
    }
    println!(
        "{}",
        if odds == 0 || (odds - 1) % 2 == 0 {
            "First"
        } else {
            "Second"
        }
    );
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
