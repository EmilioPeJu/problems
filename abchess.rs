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
    let mut wscore: usize = 0;
    let mut bscore: usize = 0;
    for _ in 0..8 {
        let line: String = scan.next();
        for c in line.chars() {
           let score: usize =  match c.to_ascii_lowercase() {
                'q' => 9,
                'r' => 5,
                'b' => 3,
                'n' => 3,
                'p' => 1,
                 _  => 0
            };
           if c.is_ascii_uppercase() { wscore += score; }
           else                      { bscore += score; }
        }
    }
    println!("{}", if wscore > bscore      {"White"} 
                   else if wscore < bscore {"Black"}
                   else                    {"Draw"});
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
