struct Scan {
    buffer: std::collections::VecDeque<String>
}

impl Scan {
    fn new() -> Scan {
        Scan { buffer: std::collections::VecDeque::new() }
    }

    fn next<T: std::str::FromStr>(&mut self)-> T {
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

// actually bfs is better for this problem, but dfs is faster to implement
fn dfs_moves(acc: usize, current: usize, goal: usize, dist: &mut [usize;20000]) {
    if dist[current]<=acc { return; }
    dist[current] = acc;
    if current >= goal {
        dist[goal] = dist[goal].min(acc+current-goal);
        return;
    }
    dfs_moves(acc + 1, current*2, goal, dist);
    if current > 1 {
        dfs_moves(acc + 1, current-1, goal, dist);
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    if m<= n {
        println!("{}", n - m);
        return;
    }
    let mut dist = [std::usize::MAX; 20000];
    dfs_moves(0, n, m, &mut dist);
    println!("{}", dist[m]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
