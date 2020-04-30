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

fn sumdigits(x: i64) -> usize {
    let mut result = 0usize;
    let mut tmp = x;
    while tmp > 0 {
        result += (tmp % 10) as usize;
        tmp /= 10;
    }
    result
}

fn _main() {
    let mut scan = Scan::new();
    let a: u32 = scan.next();
    let b: isize = scan.next();
    let c: isize = scan.next();
    let mut result: Vec<i64> = vec![];
    for i in 0usize..=81 {
        let x: i64 = (i.pow(a) as i64) * (b as i64) + (c as i64);
        if x <= 0 {
            continue;
        }
        if x >= 1000000000 {
            break;
        }
        if sumdigits(x) == i {
            result.push(x);
        }
    }
    println!("{}", result.len());
    for &item in &result {
        print!("{} ", item);
    }
    if result.len() != 0 {
        println!();
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
