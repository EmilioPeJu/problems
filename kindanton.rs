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
    let t: usize = scan.next();
    for _ in 0..t {
        let n: usize = scan.next();
        let a: Vec<isize> = scan.next_n(n);
        let b: Vec<isize> = scan.next_n(n);
        if a[0] != b[0] {
            println!("NO");
            continue;
        }
        let mut ap: Vec<isize> = a
            .clone()
            .into_iter()
            .map(|x| if x == -1 { 2 } else { x })
            .collect();
        for i in 1..n {
            ap[i] |= ap[i - 1];
        }
        let mut valid = true;
        for i in 1..n {
            if (a[i] > b[i] && ap[i - 1] & 2 == 0) || (a[i] < b[i] && ap[i - 1] & 1 == 0) {
                valid = false;
                break;
            }
        }
        println!("{}", if valid { "YES" } else { "NO" });
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
