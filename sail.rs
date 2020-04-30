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
    let t: usize = scan.next();
    let mut sx: isize = scan.next();
    let mut sy: isize = scan.next();
    let ex: isize = scan.next();
    let ey: isize = scan.next();
    let line: String = scan.next();
    if sx == ex && sy == ey {
        println!("0");
        return;
    }
    let mut result = -1isize;
    for (i, c) in line.chars().enumerate().take(t) {
        match c {
            'N' if ey > sy => {
                sy += 1;
            }
            'S' if ey < sy => {
                sy -= 1;
            }
            'W' if ex < sx => {
                sx -= 1;
            }
            'E' if ex > sx => {
                sx += 1;
            }
            _ => {}
        }
        if sx == ex && sy == ey {
            result = (i + 1) as isize;
            break;
        }
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
