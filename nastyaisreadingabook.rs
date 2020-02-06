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
    let n: usize = scan.next();
    let mut chapters: Vec<usize> = Vec::new();
    let mut i = 0;
    chapters.push(0);
    for _ in 0..n {
        let l: usize = scan.next();
        let r: usize = scan.next();
        // this could be simpler: I could have used only 'l'
        chapters.push(chapters[i] + r - l + 1);
        i += 1;
    }
    let k: usize = scan.next::<usize>() - 1;
    let pos = chapters
        .binary_search_by(|&x| {
            if x <= k {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unwrap_err()
        - 1;
    println!("{}", n - pos);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
