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

// we might as well emulate the process, but I
// wanted a dp solution (damm tag)
fn maxval(a1: usize, a2: usize, depth: usize, table: &mut [[usize; 256]; 256]) -> usize {
    if a1 == 0 || a2 == 0 {
        return depth;
    }
    if table[a1][a2] != 0 {
        return table[a1][a2];
    }
    let mut result = 0usize;
    if a2 >= 2 {
        let val = {
            let a1 = a1 + 1;
            let a2 = a2 - 2;
            maxval(a1.max(a2), a1.min(a2), depth + 1, table)
        };
        result = result.max(val);
    }
    if a1 >= 2 {
        let val = {
            let a1 = a1 - 2;
            let a2 = a2 + 1;
            maxval(a1.max(a2), a1.min(a2), depth + 1, table)
        };
        result = result.max(val);
    }
    table[a1][a2] = result;
    result
}

fn _main() {
    let mut table = [[0; 256]; 256];
    let mut scan = Scan::new();
    let a1: usize = scan.next();
    let a2: usize = scan.next();
    let result = maxval(a1.max(a2), a1.min(a2), 0, &mut table);
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
