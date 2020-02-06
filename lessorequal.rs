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
    let k: usize = scan.next();
    let mut arr: Vec<usize> = scan.next_n(n);
    arr.sort();
    if k == n {
        println!("{}", arr[n - 1]);
        return;
    }
    if k == 0 {
        if arr[0] == 1 {
            println!("-1");
        } else {
            println!("{}", arr[0] - 1);
        }
        return;
    }
    if arr[k - 1] != arr[k] {
        println!("{}", arr[k - 1]);
    } else {
        println!("-1");
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
