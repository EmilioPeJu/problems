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
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut a: Vec<i64> = scan.next_n(n);
    let mut global_inc = 0i64;
    for _ in 0..m {
        let typ: u8 = scan.next();
        match typ {
            1 => {
                let v: usize = scan.next();
                let x: usize = scan.next();
                a[v - 1] = x as i64 - global_inc;
            }
            2 => {
                let y: usize = scan.next();
                global_inc += y as i64;
            }
            3 => {
                let q: usize = scan.next();
                println!("{}", a[q - 1] + global_inc);
            }
            _ => {}
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
