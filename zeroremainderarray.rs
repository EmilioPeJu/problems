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

fn _main() -> std::io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        let n: usize = scan.next();
        let k: usize = scan.next();
        let arr: Vec<usize> = scan.next_n(n);
        let mina: usize = *arr.iter().min().unwrap();
        let maxa: usize = *arr.iter().max().unwrap();
        let candidatek: usize = (maxa / k * k) + if maxa % k != 0 { 1 } else { 0 };
        let needed = candidatek - mina;
        // 0 + 1 + ... + steps
        // sum = steps * (steps + 1) / 2 = needed
        // steps^2 + steps - 2needed = 0
        // steps = (-1 + sqrt(1+8needed))/2
        let steps: f64 = (-1f64 + (1f64 + 8f64 * (needed as f64)).sqrt()) / 2f64;
        let result: usize = steps.ceil() as usize;
        println!("{}", result);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap()?;
    Ok(())
}
