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

fn main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let jobs: Vec<usize> = scan.next_n(m);
    let mut result: u64 = 0;
    let mut current = 0;
    for &item in jobs.iter() {
        if item - 1 == current { continue; }
        if item - 1 > current {
            result +=  (item - 1 - current) as u64;
            current = item - 1;
        } else {
            result += (n - current + item - 1) as u64;
            current = item - 1;
        }
    }
    println!("{}", result);
}
