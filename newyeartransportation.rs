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
    let t: usize =  scan.next::<usize>() - 1;
    let mut graph: Vec<usize> = (0..n-1)
        .map(|index| { index + scan.next::<usize>() })
        .collect();
    graph.push(n-1);
    let mut index = 0;
    let result: bool = loop {
        if index == t { break true; }
        if index == n - 1 { break false; }
        index = graph[index];
    };
    println!("{}", if result { "YES" } else { "NO" });
}
