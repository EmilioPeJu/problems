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
    let mut n: u64 = scan.next();
    n -= 1;
    let mut current: u64 = 5;
    while current <= n {
        n -= current;
        current*=2;
    }
    let each = current / 5;
    let index = (n/each) as usize;
    let persons: [&str; 5] = ["Sheldon", "Leonard", "Penny", "Rajesh", "Howard"];
    println!("{}", persons[index]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
