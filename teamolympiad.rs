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
    let mut cnt: [Vec<usize>;3] = [vec![], vec![], vec![]];
    for i in 1..=n {
        let b: usize = scan.next();
        cnt[b-1].push(i);
    }
    let nresult =  cnt[0].len().min(cnt[1].len()).min(cnt[2].len());
    println!("{}", nresult);
    for i in 0..nresult {
        println!("{} {} {}", cnt[0][i], cnt[1][i], cnt[2][i]);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
