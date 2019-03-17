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

#[derive(Debug)]
struct Lap(u64, u64);

fn _main() {
    let mut scan = Scan::new();
    let n: u64 = scan.next();
    // 1 2
    // 2 1
    // 3 1
    let mut laptops: Vec<Lap> = Vec::new();
    for _ in 0..n {
        laptops.push(Lap(scan.next::<u64>(), scan.next::<u64>()));
    }
    if n == 1 {
        println!("Poor Alex");
        return;
    }
    laptops.sort_by(|a,b| ((a.1)*(b.0)).partial_cmp(&(b.1*a.0)).unwrap());
    let first = laptops.first().unwrap();
    let last = laptops.last().unwrap();
    if first.0 > last.0 && first.1 < last.1 {
        println!("Happy Alex");
    } else {
        println!("Poor Alex");
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
