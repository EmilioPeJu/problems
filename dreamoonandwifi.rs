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
    let sent: String = scan.next();
    let received: String = scan.next();
    let mut pos: isize = 0;
    for mov in sent.chars() {
        if mov == '+' {
            pos += 1;
        } else if mov == '-' {
            pos -= 1;
        }
    }
    let mut new_pos: isize = 0;
    let mut unknown: isize = 0;
    for mov in received.chars() {
        if mov == '+' {
            new_pos += 1;
        } else if mov == '-' {
            new_pos -= 1;
        } else {
            unknown += 1;
        }
    }
    let needed = pos - new_pos;
    if needed > unknown {
        println!("{:.10}", 0.0f64);
    } else {
        let total: usize = 2usize.pow(unknown as u32);
        let mut valid = 0isize;
        // another approach could be using dfs
        for i in 0..total {
            if (2 * i.count_ones() as isize - unknown as isize) == needed {
                valid += 1;
            }
        }
        // another approach could be using combinations: C(unknown, |needed|)/total
        let result = (valid as f64) / (total as f64);
        println!("{:.10}", result);
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
