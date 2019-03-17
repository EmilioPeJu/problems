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
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let m: usize = scan.next();
    let mut cnt = vec![0isize; input.len()];
    let mut acc = vec![0isize; input.len()];
    for i in 0..cnt.len()-1 {
        if input[i] == input[i+1] { cnt[i] = 1; }
    }
    acc[0] = cnt[0];
    for i in 1..cnt.len() {
        acc[i] = acc[i-1] + cnt[i];
    }
    for _ in 0..m {
        let (l, r) = (scan.next::<usize>()-1, scan.next::<usize>()-1);
        let result = if r==0 {0}
                     else {
                         acc[r-1] -
                             if l==0 {0}
                             else { acc[l-1] }
                     };
        println!("{}", result);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
