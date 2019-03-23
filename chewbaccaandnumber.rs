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
    let mut num: u64 = scan.next();
    let mut result: u64 = 0;
    let mut position: u64 = 1;
    while num/10 > 0 {
        let dig = num%10;
        result += dig.min(9-dig)*position;
        position *= 10;
        num /= 10;
    }
    if num == 9 {
        result += 9*position;
    } else {
        let dig = num%10;
        result += dig.min(9-dig)*position;
    }

    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
