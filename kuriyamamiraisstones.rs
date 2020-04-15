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

fn presum(arr: &mut [u64]) {
    for i in 1..arr.len() {
        arr[i] += arr[i - 1];
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut arr: Vec<u64> = scan.next_n(n);
    let mut pre_arr = arr.clone();
    arr.sort();
    presum(&mut pre_arr);
    presum(&mut arr);
    let m: usize = scan.next();
    for _ in 0..m {
        let typ: usize = scan.next();
        let l: usize = scan.next();
        let r: usize = scan.next();
        match typ {
            1 => {
                println!(
                    "{}",
                    pre_arr[r - 1] - if l > 1 { pre_arr[l - 2] } else { 0 }
                );
            }
            2 => {
                println!("{}", arr[r - 1] - if l > 1 { arr[l - 2] } else { 0 });
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
