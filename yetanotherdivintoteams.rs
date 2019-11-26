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
    let q: usize = scan.next();
    for _ in 0..q {
        let n: usize = scan.next();
        let mut arr: Vec<usize> = scan.next_n(n);
        // if n == 1 {
        //     println!("1");
        //     continue;
        // }
        arr.sort();
        let mut needmore = false;
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] + 1 {
                needmore = true;
                break;
            }
        }
        println!("{}", if needmore { "2" } else { "1" });
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
