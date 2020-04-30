use std::collections::HashSet;

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
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut notvalid: HashSet<(usize, usize)> = HashSet::new();
    let mut ends: HashSet<usize> = HashSet::new();
    for _ in 0..m {
        let a: usize = scan.next();
        let b: usize = scan.next();
        let a1 = a.min(b);
        let b1 = a.max(b);
        ends.insert(a1);
        ends.insert(b1);
        notvalid.insert((a1, b1));
    }
    let mut chosen = 0usize;
    for i in 1..=n {
        if !ends.contains(&i) {
            chosen = i;
            break;
        }
    }
    println!("{}", n - 1);
    for i in 1..=n {
        if i != chosen {
            println!("{} {}", chosen, i);
        }
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
