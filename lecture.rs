use std::collections::HashMap;

struct Scan {
    buffer: std::collections::VecDeque<String>
}

impl Scan {
    fn new() -> Scan {
        Scan { buffer: std::collections::VecDeque::new() }
    }

    fn next<T: std::str::FromStr>(&mut self)-> T {
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
    let m: usize = scan.next();
    let mut dict: HashMap<String, String> = HashMap::new();
    for _ in 0..m {
        let first: String = scan.next();
        let second: String = scan.next();
        dict.insert(first, second);
    }
    for _ in 0..n {
        let word: String = scan.next();
        let trasl = dict.get(&word).unwrap();
        if trasl.len() < word.len() {
            print!("{} ", *trasl);
        } else {
            print!("{} ", word);
        }
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
