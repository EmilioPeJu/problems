struct Scan {
    buffer: std::collections::VecDeque<String>
}

impl Scan {
    fn new() -> Scan {
        Scan { buffer: std::collections::VecDeque::new() }
    }

    fn next<T: std::str::FromStr>(&mut self)-> T {
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
    let n: usize = scan.next();
    let k: usize = scan.next();
    let hi: Vec<usize> = scan.next_n(n);
    if n == k {
        println!("1");
        return;
    }
    let mut sum: usize = hi.iter().take(k).sum();
    let mut minsum = sum;
    let mut minindex = 0;
    for i in 1..=(n-k) {
        sum = sum - hi[i-1] + hi[i+k-1];
        if sum < minsum {
            minsum = sum;
            minindex = i;
        }
    }
    println!("{}", minindex + 1);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
