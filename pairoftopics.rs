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
    let n: usize = scan.next();
    let a: Vec<isize> = scan.next_n(n);
    let b: Vec<isize> = scan.next_n(n);
    let diff: Vec<isize> = (0..n).map(|i| a[i] - b[i]).collect();
    let mut minusdiff: Vec<isize> = diff.iter().map(|x| -x).collect();
    minusdiff.sort();
    let mut result = 0u64;
    for i in 0..n {
        let current = diff[i];
        let mut low = 0usize;
        let mut high = n;
        while low < high {
            let middle = (low + high) / 2;
            if minusdiff[middle] < current {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        result += low as u64;
        if low > 0 && -current < current {
            result -= 1;
        }
    }
    println!("{}", result / 2);
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
