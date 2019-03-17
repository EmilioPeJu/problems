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
    let mut maxv: usize = 0;
    let mut maxcnt: usize = 0;
    let mut minv: usize = std::usize::MAX;
    let mut mincnt: usize = 0;
    for _ in 0..n {
        let current: usize = scan.next();
        if current < minv {
            minv = current;
            mincnt = 1; 
        } else if current == minv {
            mincnt += 1;
        }
        if current > maxv {
            maxv = current;
            maxcnt = 1; 
        } else if current == maxv {
            maxcnt += 1;
        }
    }
    if maxv == minv {
        let result: u64 = mincnt as u64*(mincnt as u64-1)/2;
        println!("0 {}", result);
    } else {
        println!("{} {}", maxv - minv, maxcnt as u64 * mincnt as u64);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
