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
}

fn _main() {
    let mut scan = Scan::new();
    let tc: usize = scan.next();
    for _ in 0..tc {
        let mut b: usize = scan.next();
        let mut p: usize = scan.next();
        let mut f: usize = scan.next();
        let mut h: usize = scan.next();
        let mut c: usize = scan.next();
        b /= 2;
        if c > h {
            std::mem::swap(&mut p, &mut f);
            std::mem::swap(&mut h, &mut c);
        }
        let result = {
            let cando = p.min(b);
            b -= cando;
            cando * h + f.min(b) * c
        };
        println!("{}", result);
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
