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
    let mut parent = [0usize; 1024];
    let mut havechild = [false; 1024];
    let mut count = [0usize; 1024];
    let mut scan = Scan::new();
    let n: usize = scan.next();
    for i in 2..=n {
        let p: usize = scan.next();
        parent[i] = p;
        havechild[p] = true;
    }
    for i in 2..=n {
        if !havechild[i] {
            count[parent[i]] += 1;
        }
    }
    for i in 1..=n {
        if havechild[i] && count[i] < 3 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
