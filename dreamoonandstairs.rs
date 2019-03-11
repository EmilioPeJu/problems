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
    let m: isize = scan.next();
    let mut twomoves: isize = n / 2;
    let mut onemoves: isize = n % 2;
    for _ in 0..m {
        if (twomoves + onemoves ) % m == 0 {
            println!("{}", twomoves+onemoves);
            return;
        }
        if twomoves > 0 {
            twomoves -= 1;
            onemoves += 2;
        }
    }
    println!("-1");
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
