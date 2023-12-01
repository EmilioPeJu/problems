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
    let a: Vec<usize> = scan.next_n(n);
    let b: Vec<usize> = scan.next_n(n);
    let mut shifts: Vec<usize> = vec![0; n + 1];
    let mut a_pos: Vec<usize> = vec![0; n + 1];
    let mut b_pos: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        a_pos[a[i]] = i;
        b_pos[b[i]] = i;
    }
    let mut maxshift = 0usize;
    for i in 1..=n {
        let currentshift = if b_pos[i] >= a_pos[i] {
            b_pos[i] - a_pos[i]
        } else {
            n - a_pos[i] + b_pos[i]
        };
        shifts[currentshift] += 1;
        maxshift = maxshift.max(shifts[currentshift]);
    }
    println!("{}", maxshift);
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
