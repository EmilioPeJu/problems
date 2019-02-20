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
    let n = scan.next();
    let input: Vec<[isize;2]> = (0..n)
        .map(|_| [scan.next(), scan.next()])
        .collect();
    let mut res: usize = 0;
    for i in 0..n {
        for j in  0..n {
            if i == j { continue; }
            if input[i][0] == input[j][1] { res += 1; }
        }
    }
    println!("{}", res);
}
