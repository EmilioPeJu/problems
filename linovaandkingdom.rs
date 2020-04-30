type Graph = Vec<Vec<usize>>;
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

fn dfs(
    graph: &Graph,
    current: usize,
    parent: usize,
    depth: &mut Vec<usize>,
    size: &mut Vec<usize>,
) {
    depth[current] = depth[parent] + 1;
    size[current] = 1;
    for &nei in graph[current].iter() {
        if nei != parent {
            dfs(graph, nei, current, depth, size);
            size[current] += size[nei];
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let k: usize = scan.next();
    let mut graph: Graph = vec![vec![]; n + 1];
    for _ in 0..(n - 1) {
        let u: usize = scan.next();
        let v: usize = scan.next();
        graph[u].push(v);
        graph[v].push(u);
    }
    if k == n {
        println!("0");
        return;
    }
    let mut depth = vec![0usize; n + 1];
    let mut size = vec![0usize; n + 1];
    dfs(&graph, 1, 1, &mut depth, &mut size);
    let mut candidates: Vec<isize> = (1..=n)
        .map(|x| (depth[x] as isize - (size[x] as isize)))
        .collect();
    candidates.sort();
    let mut result = 0i64;
    for &item in candidates.iter().rev().take(k) {
        result += item as i64;
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
