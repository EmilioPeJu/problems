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

fn gcd(mut a: usize, mut b: usize) -> usize {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let new_b = a % b;
        a = b;
        b = new_b;
    }
    if a == 0 {
        1
    } else {
        a
    }
}

fn _main() {
    let mut scan = Scan::new();
    let a: usize = scan.next();
    let b: usize = scan.next();
    let x = 6 - a.max(b) + 1;
    let common = gcd(x, 6);
    println!("{}/{}", x / common, 6 / common);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
