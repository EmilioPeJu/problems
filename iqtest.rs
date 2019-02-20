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

    fn read_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

}

fn main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let data: Vec<usize> = scan.read_n(n);
    let current = data[0] & 1;
    let mut i_current = 0;
    for (index, &item) in data.iter().enumerate() {
        if item & 1 != current {
            i_current = index;
            break;
        }
    }
    if i_current == 1 && current != (data.last().unwrap()&1) { i_current = 0; }
    println!("{}", i_current + 1);
}
