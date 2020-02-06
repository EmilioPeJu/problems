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
    let ts: usize = scan.next();
    for _ in 0..ts {
        // This is a more general solution considering the whole length
        // of the display.
        // But it's actually simpler,
        // the maximum input is less than the length of the display
        // that means we only need to consider ones (and one seven if
        // n is odd)
        let mut n: usize = scan.next();
        let mut n_one = (n / 2).min(998244353);
        n -= n_one * 2;
        let mut n_seven = n.min(998244353);
        n -= n_seven;
        n_one -= n_seven;
        let n_nine = (n / 3).min(998244353);
        if n_nine <= n_seven {
            n_seven -= n_nine;
        } else {
            n_one -= n_nine - n_seven;
            n_seven = 0;
        }
        println!(
            "{}{}{}",
            "9".repeat(n_nine),
            "7".repeat(n_seven),
            "1".repeat(n_one)
        );
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
