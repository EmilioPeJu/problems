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
    let mut cnt = [0usize; 3];
    let ex_msg = ["chest", "biceps", "back"];
    let mut maxa = 0usize;
    let mut maxi = 0usize;
    for i in 0..n {
        let a: usize = scan.next();
        let exi = i % 3;
        cnt[exi] += a;
        if cnt[exi] > maxa {
            maxa = cnt[exi];
            maxi = exi;
        }
    }
    println!("{}", ex_msg[maxi]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
