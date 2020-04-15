struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next_line(&self) -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Fail to read");
        line
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let line = self.next_line();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    let mut c1 = ['x'; 2];
    let mut c2 = ['x'; 2];
    for _ in 0..ts {
        let mut ci = 0usize;
        let n: usize = scan.next();
        let s1: Vec<char> = scan.next_line().chars().take(n).collect();
        let s2: Vec<char> = scan.next_line().chars().take(n).collect();
        for i in 0..n {
            if s1[i] != s2[i] {
                if ci >= 2 {
                    ci += 1;
                    break;
                }
                c1[ci] = s1[i];
                c2[ci] = s2[i];
                ci += 1;
            }
        }
        let result = ci == 2 && c1[0] == c1[1] && c2[0] == c2[1];
        println!("{}", if result { "Yes" } else { "No" });
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
