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
    let n: u64 = scan.next();
    let m: u64 = scan.next();
    let maxcalc = (n-m+1) * (n-m) / 2;
    let minval1 = n/m + 1;
    let minval2 = n/m;
    let minn1 = n%m;
    let minn2 = m - minn1;
    let mincalc = (minval1*(minval1-1)/2)*minn1 
        + (minval2*(minval2-1)/2)*minn2;
    println!("{} {}", mincalc, maxcalc);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
