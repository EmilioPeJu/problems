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
    // example 
    // 1 2 4 6
    // 1 5 5 7 9
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut a: Vec<isize> = scan.next_n(n);
    let m: usize = scan.next();
    let mut b: Vec<isize> = scan.next_n(m);
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut result: usize = 0;
    a.sort();
    b.sort();
    while i<n && j<m {
        if (a[i] - b[j]).abs() <= 1 {
            result += 1;
            i += 1;
            j += 1;
        } else if a[i] < b[j] { i += 1; }
          else if a[i] > b[j] { j += 1; }
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
