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
    let mut primes: [bool; 1000001] = [true; 1000001];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=1000 {
        let mut j: usize = 2;
        while i*j < 1000001 {
            primes[i*j] = false;
            j += 1;
        }
    }

    for _ in 0..n {
        let current: f64 = scan.next();
        let sqcurrent = current.sqrt();
        let sqcurrent2 = (sqcurrent as u64) as f64;
        if sqcurrent == sqcurrent2 && sqcurrent.powi(2) == current && primes[sqcurrent2 as usize] {
            println!("YES");
        } else {
            println!("NO"); 
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
