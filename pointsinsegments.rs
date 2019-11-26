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
    // a better solution would be by prefix sums calculation
    //  which would be O(n+m) instead of O(n*m)
    // create a counter array
    // for each interval: cnt[l]++; cnt[r+1]--;
    // iterate doing: cnt[i] += cnt[i-1]
    // result is every position that is 0
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut result: Vec<usize> = Vec::new();
    let mut coords = [false; 128];
    for _ in 0..n {
        let l: usize = scan.next();
        let r: usize = scan.next();
        for i in l..=r {
            coords[i] = true;
        }
    }
    for i in 1..=m {
        if !coords[i] {
            result.push(i);
        }
    }
    println!("{}", result.len());
    for &val in result.iter() {
        print!("{} ", val);
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
