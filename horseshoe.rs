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
    let a: usize = scan.next();
    let b: usize = scan.next();
    let c: usize = scan.next();
    let d: usize = scan.next();
    let eq: [bool;3]  = [a==b || a==c || a==d, b==c || b==d, c==d];
    let result: usize = eq.iter().map(|&x| (x as usize))
        .fold(0,|sum, x| sum+x);
    println!("{}", result);
}
