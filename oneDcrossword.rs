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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let _n: usize = scan.next();
    let input: Vec<char> = scan.next::<String>().chars().take(_n).collect();
    let mut results: Vec<usize> = vec![];
    let mut count: usize = 0;
    for &item in input.iter() {
        if item == 'B' {
            count += 1;
        }
        if item == 'W' {
            if count != 0 {
                results.push(count);
            }
            count = 0;
        }
    }
    if count != 0 {
        results.push(count);
    }
    println!("{}", results.len());
    for &item in results.iter() {
        print!("{} ", item);
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
