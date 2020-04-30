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
}

fn get_order(x: usize, y: usize) -> usize {
    if x == 0 {
        return 0;
    }
    let mut x = x;
    let mut count = 0usize;
    while x % y == 0 {
        x /= y;
        count += 1;
    }
    count
}

fn _main() {
    let mut scan = Scan::new();
    let mut a: usize = scan.next();
    let mut b: usize = scan.next();
    let mut result = 0usize;
    let possible: Vec<usize> = vec![2, 3, 5];
    for i in possible {
        let ia = get_order(a, i);
        let ib = get_order(b, i);
        let diff = ia.max(ib) - ia.min(ib);
        result += diff;
        if ia > ib {
            a /= i.pow(diff as u32);
        } else if ia < ib {
            b /= i.pow(diff as u32);
        }
    }
    if a != b {
        println!("-1");
    } else {
        println!("{}", result);
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
