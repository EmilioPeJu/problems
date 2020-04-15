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

struct Uf {
    count: Vec<usize>,
    parent: Vec<usize>,
}

impl Uf {
    fn new(size: usize) -> Uf {
        Uf {
            count: vec![1usize; size],
            parent: (0..size).collect(),
        }
    }

    fn root(&self, a: usize) -> usize {
        let mut current = a;
        while self.parent[current] != current {
            current = self.parent[current];
        }
        current
    }

    fn connected(&self, a: usize, b: usize) -> bool {
        return self.root(a) == self.root(b);
    }

    fn connect(&mut self, a: usize, b: usize) {
        let ra = self.root(a);
        let rb = self.root(b);
        if self.count[ra] > self.count[rb] {
            self.parent[rb] = ra;
            self.count[ra] += self.count[rb];
        } else {
            self.parent[ra] = rb;
            self.count[rb] += self.count[ra];
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut allzero = 0usize;
    // 0 to n-1 are people
    // n to n + m - 1 are languages
    let mut uf = Uf::new(n + m);
    for i in 0..n {
        let s: usize = scan.next();
        allzero |= s;
        for _ in 0..s {
            let lan: usize = scan.next();
            uf.connect(i, lan - 1 + n);
        }
    }
    let mut result = 0usize;
    for i in 1..n {
        if !uf.connected(i, 0) {
            result += 1;
            uf.connect(i, 0);
        }
    }
    if allzero == 0 {
        // person 0 must speak at least 1 language
        result += 1;
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
