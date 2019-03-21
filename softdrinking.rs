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
    let k: usize = scan.next();
    let l: usize = scan.next();
    let c: usize = scan.next();
    let d: usize = scan.next();
    let p: usize = scan.next();
    let nl: usize = scan.next();
    let np: usize = scan.next();
    let l_person = l*k/nl;
    let c_person = c*d;
    let p_person = p/np;
    let result = l_person.min(c_person).min(p_person)/n;
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
