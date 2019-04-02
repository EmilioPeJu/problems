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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let mine = scan.next::<String>();
    let mine = mine.as_bytes();
    let table: Vec<String> = scan.next_n(5);
    let mut result = false;
    for card in table.into_iter() {
        let x = card.as_bytes();
        if x[0] == mine[0] || x[1] == mine[1] {
            result = true;
            break;
        }
    }
    println!("{}", if result { "YES" } else { "NO" });
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
