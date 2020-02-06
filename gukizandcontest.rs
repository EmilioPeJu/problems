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
    // this is O(n), however the brute force solution O(n^2) would also pass
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let arr: Vec<usize> = scan.next_n(n);
    let mut sum = [0usize; 2048];
    let mut max_rating = 0usize;
    for &item in arr.iter() {
        sum[item] += 1;
        max_rating = max_rating.max(item);
    }
    for i in (0..max_rating).rev() {
        sum[i] += sum[i + 1];
    }
    for &item in arr.iter() {
        print!("{} ", sum[item + 1] + 1);
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
