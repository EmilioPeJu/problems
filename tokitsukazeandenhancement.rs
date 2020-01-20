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

fn get_class(x: usize) -> usize {
    let classes = [0usize, 2, 3, 1];
    return classes.iter().position(|&v| v == (x % 4)).unwrap();
}

fn _main() {
    let mut scan = Scan::new();
    let x: usize = scan.next();
    let mut maxclass = get_class(x);
    let mut maxb = 0;
    for &step in [1usize, 2].iter() {
        let current_class = get_class(x + step);
        if current_class > maxclass {
            maxclass = current_class;
            maxb = step;
        }
    }
    println!("{} {}", maxb, ['D', 'C', 'B', 'A'][maxclass]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
