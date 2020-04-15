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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let t: usize = scan.next();
    for _ in 0..t {
        let mut counter: HashMap<usize, usize> = HashMap::new();
        let n: usize = scan.next();
        let arr: Vec<usize> = scan.next_n(n);
        for item in &arr {
            counter.entry(*item).and_modify(|x| *x += 1).or_insert(1);
        }
        let centry = counter.iter().max_by_key(|x| x.1).unwrap();
        let mut rest = counter.len() - 1;
        let mut repcount = *centry.1;
        if repcount > rest {
            repcount -= 1;
            rest += 1;
        }
        println!("{}", repcount.min(rest));
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
