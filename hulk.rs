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
    let n: usize = scan.next();
    for i in 0..n-1 {
        if i%2==0 { print!("I hate that "); }
        else      { print!("I love that "); }
    }
    if (n-1)%2==0 { println!("I hate it"); }
    else            { println!("I love it"); }
}
