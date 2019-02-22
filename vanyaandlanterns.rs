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
            std::io::stdin()
                .read_line(&mut line)
                .expect("Fail to read");
            self.buffer = line
                .split_whitespace()
                .map(String::from)
                .collect();
        }
    }

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| self.next::<T>())
            .collect()
    }

}

fn main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let l: usize = scan.next();
    let mut dist: Vec<usize> = scan.next_n(n);
    dist.sort();
    let mut result: f64 = dist[0] as f64;
    for i in 0..n-1 {
        let required_d: f64 = (dist[i+1] - dist[i]) as f64 / 2f64;
        result = result.max(required_d);
    }
    result = result.max((l - *dist.last().unwrap()) as f64);
    println!("{:.10}", result);
}
