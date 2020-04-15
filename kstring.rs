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
    let k: usize = scan.next();
    let mut counter: HashMap<char, usize> = HashMap::new();
    let line: String = scan.next();
    for c in line.chars() {
        counter.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut possible = true;
    let mut sresult: Vec<String> = vec![];
    for (key, val) in counter {
        if val % k != 0 {
            possible = false;
            break;
        } else {
            sresult.push(key.to_string().repeat(val / k));
        }
    }
    let result = sresult.join("").repeat(k);
    println!("{}", if !possible { "-1".to_string() } else { result });
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
