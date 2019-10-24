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
    let r: usize = scan.next();
    let c: usize = scan.next();
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..r {
        let input: Vec<char> = scan.next::<String>().chars().take(c).collect();
        grid.push(input);
    }
    let mut rcount = 0usize;
    let mut ccount = 0usize;
    for i in 0..r {
        let mut cake = true;
        for j in 0..c {
            cake = cake & (grid[i][j] == '.');
        }
        if cake {
            rcount += 1;
        }
    }
    for j in 0..c {
        let mut cake = true;
        for i in 0..r {
            cake = cake & (grid[i][j] == '.');
        }
        if cake {
            ccount += 1;
        }
    }
    let result = rcount * c + ccount * r - rcount * ccount;
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
