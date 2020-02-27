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
        let mut n: usize = scan.next();
        let mut cnt = 0usize;
        let mut first = 0usize;
        let mut second = 0usize;
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                n /= i;
                cnt += 1;
                if cnt == 1 {
                    first = i;
                } else if cnt == 2 {
                    // it's either a prime number, or
                    // a composite with 'first' as prime factor.
                    // In the first case, n is valid, in the second
                    // one n could clash with 'second', if this happens
                    // there is no way of rearranging factors to get a
                    // different number and there is no solution.
                    second = i;
                    break;
                }
            }
            i += 1;
        }

        if cnt != 2 || n == first || n == second {
            println!("NO");
        } else {
            println!("YES");
            println!("{} {} {}", first, second, n);
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
