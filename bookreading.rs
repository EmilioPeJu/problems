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

fn get_cycle(n: u64, m: u64) -> Vec<u64> {
    // it would be easier to consider cycle with length 10
    // instead of finding the smallest cycle
    let mut result: Vec<u64> = Vec::new();
    let mut candidate = m;
    while !result.contains(&(candidate % 10)) && candidate <= n {
        result.push(candidate % 10);
        candidate += m;
    }
    result
}

fn _main() {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        let n: u64 = scan.next();
        let m: u64 = scan.next();
        let cycle = get_cycle(n, m);
        let cycle_sum = cycle.iter().sum::<u64>();
        if cycle_sum == 0 {
            println!("0");
            continue;
        }
        let mut result = (n / m) / (cycle.len() as u64) * cycle_sum;
        for i in 0..((n / m) % cycle.len() as u64) {
            result += cycle[i as usize];
        }
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
