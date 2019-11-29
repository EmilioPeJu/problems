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
    let arr: Vec<i32> = scan.next_n(n);
    let mut cnt = [0u64; 2];
    cnt[0] = (arr[0] - 1).abs() as u64;
    cnt[1] = (arr[0] + 1).abs() as u64;
    for &item in arr.iter().skip(1) {
        let x1 = cnt[0] + (item - 1).abs() as u64;
        let x2 = cnt[1] + (item + 1).abs() as u64;
        let y1 = cnt[0] + (item + 1).abs() as u64;
        let y2 = cnt[1] + (item - 1).abs() as u64;
        cnt[0] = x1.min(x2);
        cnt[1] = y1.min(y2);
    }
    println!("{}", cnt[0]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
