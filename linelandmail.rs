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
    let arr: Vec<isize> = scan.next_n(n);
    let max1 = arr.first().unwrap();
    let max2 = arr.last().unwrap();
    for i in 0..n {
        let mut minval: isize = std::isize::MAX;
        let mut maxval: isize = 0;
        maxval = maxval.max((max1 - arr[i]).abs()).max((max2 - arr[i]).abs());
        if i > 0 {
            minval = minval.min((arr[i]-arr[i-1]).abs());
        }
        if i < (n-1) {
            minval = minval.min((arr[i]-arr[i+1]).abs());
        }
        println!("{} {}", minval, maxval);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
