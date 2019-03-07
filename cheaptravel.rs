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
    let m: isize = scan.next();
    let a: isize = scan.next();
    let b: isize = scan.next();
    if a*m <= b {
        println!("{}", a*n);
    } else {
        let mut result = (n/m)*b;
        if b <= a*(n%m) { result += b; }
        else {            result += (n%m)*a; }
        println!("{}", result);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
