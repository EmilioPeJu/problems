struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next_line(&self) -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Fail to read");
        line
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let line = self.next_line();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let mut c = [0u64; 3];
    for s in scan.next::<String>().chars() {
        match s {
            'B' => {
                c[0] += 1;
            }
            'S' => {
                c[1] += 1;
            }
            'C' => {
                c[2] += 1;
            }
            _ => {}
        }
    }
    let n: Vec<u64> = scan.next_n(3);
    let p: Vec<u64> = scan.next_n(3);
    let r: u64 = scan.next();
    let mut low = 0u64;
    let mut high = 1000000001000u64;
    while low <= high {
        let middle = (low + high) / 2;
        let mut f = 0u64;
        for i in 0..3 {
            if c[i] * middle > n[i] {
                f += (c[i] * middle - n[i]) * p[i];
            }
        }
        if f <= r {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }
    println!("{}", low - 1);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
