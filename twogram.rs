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
    let _n: usize = scan.next();
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut res: String = String::new();
    let mut cnt = 0usize;
    for i in 0..input.len() - 1 {
        let current: String = input[i..=i + 1].iter().collect();
        let val = map
            .entry(current.clone())
            .and_modify(|x| {
                *x += 1;
            })
            .or_insert(1);
        if *val > cnt {
            cnt = *val;
            res = current.clone();
        }
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
