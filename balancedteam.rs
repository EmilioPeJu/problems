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
    let mut arr: Vec<isize> = scan.next_n(n);
    arr.sort();
    let mut count = 0;
    let mut left = 0;
    let mut right = 0;
    loop {
        while right < n - 1 && arr[right] - arr[left] <= 5 {
            right += 1;
        }
        count = count.max(right - left);
        if right == n - 1 {
            break;
        }
        while arr[right] - arr[left] > 5 {
            left += 1;
        }
    }
    if arr[right] - arr[left] <= 5 {
        count = count.max(right - left + 1);
    }
    println!("{}", count);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
