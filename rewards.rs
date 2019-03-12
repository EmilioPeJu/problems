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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

}
fn _main() {
    let mut scan = Scan::new();
    let sum1: usize = scan.next_n(3).iter().sum();
    let sum2: usize = scan.next_n(3).iter().sum();
    let n: usize = scan.next();
    let needed: usize = sum1/5 + sum2/10 + if sum1%5!=0 {1} else {0} 
                        + if sum2%10!=0 {1} else {0};
    println!("{}", if n>=needed {"YES"} else {"NO"});
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
