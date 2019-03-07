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
    let n: usize = scan.next();
    let points: Vec<usize> = scan.next_n(n);
    let mut result: usize = 0;
    let mut max = points[0];
    let mut min = points[0];
    for &point in  points.iter().skip(1) {
        if point > max || point < min {
            max = max.max(point);
            min = min.min(point);
            result += 1;
        }
    }
    println!("{}", result);

}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
