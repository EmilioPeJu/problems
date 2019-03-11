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
    // e.g: 3 -> 3 + (2*(2-1)+1) + 1
    // e.g: 4 -> 4 + 3 + 2 + 1 =
    let n: isize = scan.next();
    if n == 1 {
        println!("1");
        return;
    }
    let mut result: isize = n + 1; 
    let mut i = n-1;
    while i >= 2 {
        result += i*(n-i) + 1;
        i -= 1;
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
