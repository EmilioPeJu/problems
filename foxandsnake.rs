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
    let m: usize = scan.next();
    for i in 0..n {
        if i%2 == 0 {
            println!("{}", "#".repeat(m));
        } else {
            let linetype = i/2*2 % 4;
            if linetype == 0 {
                println!("{}#", ".".repeat(m-1));
            } else {
                println!("#{}", ".".repeat(m-1));
            }
        }
    }
}
