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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut wsizes: Vec<usize> = scan.next_n(n);
    let m: usize = scan.next();
    let elected: Vec<usize> = scan.next_n(m);
    for index in 1..wsizes.len() {
        wsizes[index] += wsizes[index-1];
    }
    // e.g: wsizes = [2, 4, 6]
    //      worm = 2 -> pos = 1
    for &worm in elected.iter() {
        let mut pos: usize = wsizes.binary_search_by(|&x| {
            if x < worm  { return std::cmp::Ordering::Less; }
            else         { return std::cmp::Ordering::Greater; }
        }).unwrap_err();
        println!("{}", pos + 1);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
