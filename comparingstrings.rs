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
    let first: String = scan.next();
    let second: String = scan.next();
    if first.len() != second.len() {
        println!("NO");
        return;
    }
    let mut ndiff = 0usize;
    let mut diff1 = [0u8; 3];
    let mut diff2 = [0u8; 3];
    let first_b = first.as_bytes();
    let second_b = second.as_bytes();
    for i in 0..first.len() {
        if first_b[i] != second_b[i] {
            diff1[ndiff] = first_b[i];
            diff2[ndiff] = second_b[i];
            ndiff += 1;
        }
        if ndiff > 2 {
            break;
        }
    }
    println!(
        "{}",
        if ndiff == 0 || ndiff == 2 && diff1[0] == diff2[1] && diff1[1] == diff2[0] {
            "YES"
        } else {
            "NO"
        }
    );
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
