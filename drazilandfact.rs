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
    let _: usize = scan.next();
    let num: String = scan.next();
    let mut cnt = [0usize; 10];
    for c in num.chars() {
        match c {
            '2' => {
                cnt[2] += 1;
            }
            '3' => {
                cnt[3] += 1;
            }
            '4' => {
                cnt[2] += 2;
                cnt[3] += 1;
            }
            '5' => {
                cnt[5] += 1;
            }
            '6' => {
                cnt[3] += 1;
                cnt[5] += 1;
            }
            '7' => {
                cnt[7] += 1;
            }
            '8' => {
                cnt[2] += 3;
                cnt[7] += 1;
            }
            '9' => {
                cnt[2] += 1;
                cnt[3] += 2;
                cnt[7] += 1;
            }
            _ => {}
        }
    }
    for i in (2..10).rev() {
        for _ in 0..cnt[i] {
            print!("{}", i);
        }
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
