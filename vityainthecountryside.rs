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
    let n: usize = scan.next();
    let arr: Vec<usize> = scan.next_n(n);
    if arr.len() == 1 {
        if arr[0] == 0 {
            println!("UP");
        } else if arr[0] == 15 {
            println!("DOWN");
        } else {
            println!("-1");
        }
        return;
    }
    let last = arr[arr.len() - 1];
    let last2 = arr[arr.len() - 2];
    if last2 != 14 && last != 15 && (last > last2 || last == 0 && last2 == 1) {
        println!("UP");
    } else {
        println!("DOWN");
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
