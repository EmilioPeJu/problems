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
    let n: isize = scan.next();
    if n >= 0 {
        println!("{}", n);
    } else {
        let x: String = n.to_string();
        if x.len() == 2 {
            println!("0");
        } else {
            let mut x1: String = x.clone();
            let x1len = x1.len();
            x1.remove(x1len - 1);
            let x1: isize = x1.parse().unwrap();
            let mut x2: String = x.clone();
            let x2len = x2.len();
            x2.remove(x2len - 2);
            let x2: isize = x2.parse().unwrap();
            println!("{}", x1.max(x2));
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
