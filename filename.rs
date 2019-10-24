struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
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
    let _n: usize = scan.next();
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let mut cnt = 0usize;
    let mut res = 0usize;
    if input[0] == 'x' {
        cnt += 1
    };
    for i in 1..input.len() {
        if input[i] == 'x' {
            if input[i - 1] == 'x' {
                if cnt + 1 >= 3 {
                    res += 1;
                } else {
                    cnt += 1;
                }
            } else {
                cnt = 1;
            }
        } else {
            cnt = 0;
        }
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
