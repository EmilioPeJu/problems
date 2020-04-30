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

    fn lonely(&self, a: usize) -> bool {
        self.parent[a] == a && self.count[a] == 1
    }

    fn connect(&mut self, a: usize, b: usize) -> usize {
        let ra = self.root(a);
        let rb = self.root(b);
        if ra == rb {
            return self.count[ra];
        }
        if self.count[ra] > self.count[rb] {
            self.parent[rb] = ra;
            self.count[ra] += self.count[rb];
            self.count[ra]
        } else {
            self.parent[ra] = rb;
            self.count[rb] += self.count[ra];
            self.count[rb]
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut uf = Uf::new(n + 1);
    let mut teams: Vec<Vec<usize>> = vec![Vec::new(); 49];
    // this approach is based on DFU, however, it seems
    // more intuitive to create a graph and find connected
    // components
    for _ in 0..m {
        let a: usize = scan.next();
        let b: usize = scan.next();
        let subtree = uf.connect(a, b);
        if subtree > 3 {
            println!("-1");
            return;
        }
    }
    let mut lonely: Vec<usize> = Vec::new();
    for i in 1..=n {
        if !uf.lonely(i) {
            teams[uf.root(i)].push(i);
        } else {
            lonely.push(i);
        }
    }
    for team in teams.iter_mut() {
        if team.len() == 0 {
            continue;
        }
        let left = 3 - team.len();
        if lonely.len() < left {
            println!("-1");
            return;
        }
        for _ in 0..left {
            team.push(lonely.pop().unwrap());
        }
    }
    if lonely.len() % 3 != 0 {
        println!("-1");
        return;
    }
    for team in teams.iter() {
        if team.len() != 0 {
            println!("{} {} {}", team[0], team[1], team[2]);
        }
    }
    for i in (0..lonely.len()).step_by(3) {
        println!("{} {} {}", lonely[i], lonely[i + 1], lonely[i + 2]);
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
