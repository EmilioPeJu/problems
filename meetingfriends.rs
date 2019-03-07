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
    let a: isize = scan.next();
    let b: isize = scan.next();
    let c: isize = scan.next();
    let mut result: isize = std::isize::MAX;
    // the space is too short and bruteforce is possible,
    // a better solution is max(a,b,c)-min(a,b,c)
    for i in 0..=100 {
        result = result.min((i-a).abs() + (i-b).abs() + (i-c).abs());
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
