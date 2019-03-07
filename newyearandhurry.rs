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
    let n: isize = scan.next();
    let k: isize = scan.next();
    let minutes: isize = 240 - k;
    // also can be solved by binary searching
    // but this way is cheaper
    // (5+x*5)*x / 2 >= minutes
    // 5x^2+5x-2minutes>=0
    // x=(-5+sqrt(25+40minutes))/10
    let mut result: isize = (-5+(f64::sqrt(25f64+40f64*(minutes as f64)) as isize)) / 10;
    result = result.min(n);
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
