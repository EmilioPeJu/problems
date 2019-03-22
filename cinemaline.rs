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

}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut n25: usize = 0;
    let mut n50: usize = 0;
    let mut result = true;
    for _ in 0..n {
        let current: usize = scan.next();
        if current == 25 {
            n25 += 1;
        } else if current == 50 {
            if n25 >= 1 {
                n25 -= 1;
                n50 += 1;
            } else {
                result = false;
            }
        } else { // 100
            if n50 >= 1 && n25 >= 1 {
                n50 -= 1;
                n25 -= 1;
            } else if n25 >= 3 {
                n25 -= 3;
            } else {
                result = false;
            }
        }
    }
    println!("{}", if result {"YES"} else {"NO"});
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
