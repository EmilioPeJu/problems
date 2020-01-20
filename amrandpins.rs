struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
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
    let r: i64 = scan.next();
    let x: i64 = scan.next();
    let y: i64 = scan.next();
    let xp: i64 = scan.next();
    let yp: i64 = scan.next();
    // steps = ceil(d/(2*r))
    // we could avoid floating points by using squares
    // steps^2 = d^2/(2*r)^2
    // steps^2*(2*r)^2 = d^2
    // and then use binary search to solve it
    let d = (((x - xp) * (x - xp) + (y - yp) * (y - yp)) as f64).sqrt();
    let mut steps = d as i64 / (2 * r);
    if ((steps * 2 * r) as f64) < d {
        steps += 1;
    }
    println!("{}", steps);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
