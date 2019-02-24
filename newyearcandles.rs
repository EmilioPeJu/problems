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

fn main() {
    let mut scan = Scan::new();
    let a = scan.next::<usize>();
    let b = scan.next::<usize>();
    let mut result = a;
    let mut current = a;
    while current>=b {
        result += current / b;
        current = current / b + current % b;
    }
    println!("{}", result);
}
