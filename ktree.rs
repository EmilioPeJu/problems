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
    let n: usize = scan.next();
    let k: usize = scan.next();
    let d: usize = scan.next();
    let mut dp = [[0usize; 2]; 128];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=k {
            if i < j {
                continue;
            }
            if j < d {
                // it would be better to create a function that only
                // subtruct 1000000007 when higher, this would save
                // the mod computation every time
                dp[i][0] = (dp[i][0] + dp[i - j][0]) % 1000000007;
                dp[i][1] = (dp[i][1] + dp[i - j][1]) % 1000000007;
            } else {
                dp[i][1] = (dp[i][1] + dp[i - j][0]) % 1000000007;
                dp[i][1] = (dp[i][1] + dp[i - j][1]) % 1000000007;
            }
        }
    }
    println!("{}", dp[n][1]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
