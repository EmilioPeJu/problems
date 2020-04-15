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
    let arr: Vec<usize> = scan.next_n(n);
    let mut cnt = [0usize; 8];
    for item in &arr {
        cnt[*item] += 1;
    }
    // possibles
    // 1 2 4
    // 1 2 6
    // 1 3 6
    // 1 1 1
    let n124 = cnt[1].min(cnt[2]).min(cnt[4]);
    cnt[1] -= n124;
    cnt[2] -= n124;
    cnt[4] -= n124;
    let n126 = cnt[1].min(cnt[2]).min(cnt[6]);
    cnt[1] -= n126;
    cnt[2] -= n126;
    cnt[6] -= n126;
    let n136 = cnt[1].min(cnt[3]).min(cnt[6]);
    cnt[1] -= n136;
    cnt[3] -= n136;
    cnt[6] -= n136;
    let possible = cnt.iter().all(|x| *x == 0);
    if !possible {
        println!("-1");
    } else {
        print!("{}", "1 2 4\n".repeat(n124));
        print!("{}", "1 2 6\n".repeat(n126));
        print!("{}", "1 3 6\n".repeat(n136));
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
