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

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut fives = 0usize;
    let mut zeros = 0usize;
    for _ in 0..n {
        let val: usize = scan.next();
        if val == 5 {
            fives += 1;
        } else if val == 0 {
            zeros += 1;
        }
    }
    if zeros == 0 {
        println!("-1");
        return;
    }
    if fives / 9 == 0 {
        println!("0");
        return;
    }
    println!("{}{}", "5".repeat(fives / 9 * 9), "0".repeat(zeros));
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
