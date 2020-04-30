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

fn interval(a: isize, b: isize) -> (isize, isize) {
    (a.min(b), a.max(b))
}

fn clash(inter1: (isize, isize), inter2: (isize, isize)) -> bool {
    inter1.0 < inter2.0 && inter2.0 < inter1.1 && inter1.1 < inter2.1
        || inter2.0 < inter1.0 && inter1.0 < inter2.1 && inter2.1 < inter1.1
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let arr: Vec<isize> = scan.next_n(n);
    let mut inters: Vec<(isize, isize)> = vec![];
    for i in 1..arr.len() {
        let current = interval(arr[i], arr[i - 1]);
        for &inter in &inters {
            if clash(current, inter) {
                println!("yes");
                return;
            }
        }
        inters.push(current);
    }
    println!("no");
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
