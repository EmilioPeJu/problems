
// 1 1 2  2 2 2 2 3 3
//
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
    let mut cnt: [usize; 100001] = [0; 100001];
    let mut table: [u64; 100001] = [0; 100001];
    for _ in 0..n {
        let current: usize = scan.next();
        cnt[current] += 1;
    }
    table[1] = cnt[1] as u64;
    for i in 2..=100000 {
        table[i] = table[i-1].max(table[i-2] + (i as u64)*(cnt[i] as u64));
    }
    println!("{}", table[100000]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(50_000_000).spawn(_main).unwrap().join().unwrap();
}
