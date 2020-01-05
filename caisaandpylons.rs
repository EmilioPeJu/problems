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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let h: Vec<usize> = scan.next_n(n);
    let mut energy: i64 = 0;
    let mut paid: usize = 0;
    let mut last_h = 0usize;
    for &h_i in h.iter() {
        let step: i64 = last_h as i64 - h_i as i64;
        energy += step;
        if energy < 0 {
            paid += (-energy) as usize;
            energy = 0;
        }
        last_h = h_i;
    }
    println!("{}", paid);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
