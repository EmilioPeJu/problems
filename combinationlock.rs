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
    let line1: Vec<char> = scan.next::<String>().chars().collect();
    let line2: Vec<char> = scan.next::<String>().chars().collect();
    let mut result: isize = 0;
    for index in 0..n {
        let dig1 = line1[index].to_digit(10).unwrap() as isize;
        let dig2 = line2[index].to_digit(10).unwrap() as isize;
        let needed = (dig1-dig2).abs()
            .min(10 - dig1.max(dig2) + dig1.min(dig2));
        result += needed;
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
