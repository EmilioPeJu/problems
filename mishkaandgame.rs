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
    let mut result1: usize = 0;
    let mut result2: usize = 0;
    for _ in 0..n {
        let a: usize = scan.next();
        let b: usize = scan.next();
        if a > b { result1 += 1; }
        else if a < b { result2 += 1; }
    }
    if result1 > result2 {
        println!("Mishka");
    } else if result1 < result2 {
        println!("Chris");
    } else {
        println!("Friendship is magic!^^");
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
