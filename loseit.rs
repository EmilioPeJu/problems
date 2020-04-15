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
    let valid: [usize; 6] = [4, 8, 15, 16, 23, 42];
    let arr: Vec<usize> = scan
        .next_n::<usize>(n)
        .into_iter()
        .map(|x| valid.iter().position(|y| *y == x).unwrap())
        .collect();
    let mut counter = [0usize; 6];
    for &item in &arr {
        if item == 0 {
            counter[0] += 1;
        } else {
            let diff = 1.min(counter[item - 1]);
            counter[item] += diff;
            counter[item - 1] -= diff;
        }
    }
    println!("{}", arr.len() - counter[5] * 6);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
