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
    let mut arr: Vec<usize> = scan.next_n(n);
    if n == 1 {
        println!("{}", arr[0] * arr[0]);
        return;
    }
    arr.sort();
    let mut turn = 1usize;
    let mut i = 0usize;
    let mut j = n - 1;
    let mut acc_i = 0u64;
    let mut acc_j = 0u64;
    while i <= j {
        if turn == 0 {
            acc_i += arr[i] as u64;
            i += 1;
        } else {
            acc_j += arr[j] as u64;
            j -= 1;
        }
        turn = 1 - turn;
    }
    println!("{}", acc_i * acc_i + acc_j * acc_j);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
