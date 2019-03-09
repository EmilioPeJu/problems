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
    let mut stack: Vec<char> = vec![];
    let inputstr: String = scan.next();
    for input in inputstr.chars().take(n) {
        if input == '1' {
            if let Some(&'0') = stack.last() {
                stack.pop();
            } else {
                stack.push('1');
            }
        } else { // '0'
            if let Some(&'1') = stack.last() {
                stack.pop();
            } else {
                stack.push('0');
            }
        }
    }
    let result = stack.len();
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
