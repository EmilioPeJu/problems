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
    let q: usize = scan.next();
    for _ in 0..q {
        let s: Vec<char> = scan.next_line().chars().collect();
        let t: Vec<char> = scan.next_line().chars().collect();
        let mut p: Vec<char> = scan.next_line().chars().collect();
        let mut s_i = 0;
        let mut t_i = 0;
        let mut compat = true;
        // another approach: check that s is subsequence of t
        // and check that count of each letter in s plus count of
        // that letter in p is greater or equal to count of that letter in t
        //
        // n is very small so even this n^2 approach will pass the time limit
        while s_i < s.len() && t_i < t.len() && compat {
            if s[s_i] == t[t_i] {
                s_i += 1;
                t_i += 1;
            } else {
                match p.iter().position(|&x| x == t[t_i]) {
                    None => {
                        compat = false;
                    }
                    Some(index) => {
                        p.remove(index);
                        t_i += 1;
                    }
                }
            }
        }
        println!(
            "{}",
            if compat && s_i == s.len() && t_i == t.len() {
                "YES"
            } else {
                "NO"
            }
        );
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
