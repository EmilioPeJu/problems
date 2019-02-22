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

fn main() {
    let mut scan = Scan::new();
    let mut s: usize = scan.next();
    let n: usize = scan.next();
    let mut defeated: bool = false;
    let mut dragons: Vec<[usize; 2]> = Vec::new() ;
    for _ in 0..n {
        let x: usize = scan.next();
        let y: usize = scan.next();
        dragons.push([x, y]);
    }
    dragons.sort_by(|&x, &y| x[0].partial_cmp(&y[0]).unwrap());
    for [x, y] in dragons {
        if x >= s { defeated = true; }
        else        { s += y; }
    }
    println!("{}", if defeated { "NO" } else { "YES" });
}
