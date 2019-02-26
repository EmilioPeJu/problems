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
    let a: usize = scan.next();
    let b: usize = scan.next();
    let c: usize = scan.next();
    let mut table: [usize; 4001] = [0; 4001];
    table[a] = 1;
    table[b] = 1;
    table[c] = 1;
    for index in 1..=n {
        for &from in [a, b, c].iter() {
            if index >= from && table[index-from] > 0 {
                table[index] = table[index].max(table[index - from] + 1);
            }
        }
    }
    println!("{}", table[n]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
