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

fn print_arr(x: &[isize]) {
    print!("{} ", x.len());
    for item in x {
        print!("{} ", item);
    }
    println!();
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let arr: Vec<isize> = scan.next_n(n);
    let mut first: Vec<isize> = vec![];
    let mut second: Vec<isize> = vec![];
    let mut third: Vec<isize> = vec![];
    for item in arr {
        if item > 0 {
            second.push(item);
        } else if item < 0 {
            first.push(item);
        } else {
            third.push(item);
        }
    }
    if first.len() % 2 == 0 {
        third.push(first.pop().unwrap());
    }
    while first.len() > 1 {
        second.push(first.pop().unwrap());
    }
    print_arr(&first);
    print_arr(&second);
    print_arr(&third);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
