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

fn gdc(a: isize, b: isize) -> isize {
    let mut x = a.min(b);
    let mut y = a.max(b);
    while x > 0 {
        std::mem::swap(&mut x, &mut y);
        x = x % y;
    }
    y
}

fn _main() {
    let mut scan = Scan::new();
    let arr: [isize; 2] = [scan.next(), scan.next()];
    let mut n: isize = scan.next();
    let mut index: usize = 0;
    while n >= 0 {
        let sub = gdc(n, arr[index]);
        n -= sub;
        index = 1 - index;
    }
    println!("{}", index);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
