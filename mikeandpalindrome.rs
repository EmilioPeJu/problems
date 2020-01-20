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
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let mut changes = 0;
    for i in 0..(input.len() / 2) {
        if input[i] != input[input.len() - i - 1] {
            changes += 1;
            if changes > 1 {
                println!("NO");
                return;
            }
        }
    }
    println!(
        "{}",
        if changes == 1 || changes == 0 && input.len() % 2 == 1 {
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
