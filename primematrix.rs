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

fn get_primes(till: usize) -> Vec<bool> {
    let mut primes: Vec<bool> = vec![true; till];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=(till as f64).sqrt() as usize {
        if !primes[i] {
            continue;
        }
        let mut j = 2;
        while i * j < till {
            primes[i * j] = false;
            j += 1;
        }
    }
    primes
}

fn _main() {
    let mut scan = Scan::new();
    let primes = get_primes(210000);
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut matrix: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let line: Vec<usize> = scan.next_n(m);
        matrix.push(line);
    }
    let mut changes: Vec<Vec<usize>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            let mut current = matrix[i][j];
            while !primes[current] {
                current += 1;
            }
            changes[i][j] = current - matrix[i][j];
        }
    }
    let mut result = std::usize::MAX;
    for i in 0..n {
        result = result.min(changes[i].iter().sum());
    }
    for j in 0..m {
        let mut col = 0usize;
        for i in 0..n {
            col += changes[i][j];
        }
        result = result.min(col);
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
