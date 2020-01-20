use std::cmp::Ord;

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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tree {
    x: isize,
    h: isize,
}

fn _main() {
    // this uses dynamic programming approach, however, it would
    // be better to use the greedy approach as any tree can only prevent
    // the next one by falling right, and it wouldn't make it worse (it's
    // an exchange)
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut line: Vec<Tree> = Vec::new();
    for _ in 0..n {
        let x: isize = scan.next();
        let h: isize = scan.next();
        line.push(Tree { x, h });
    }
    line.sort();
    // number of trees cut until point i, considering
    // we try the 3 actions: 0 nothing, 1 to the left, 2 to the right
    let mut dp = [[0usize; 3]; 100001];
    dp[0][0] = 0;
    dp[0][1] = 1; // safe action, asuming it's going to the left
    dp[0][2] = 0;
    let mut dp_i = 1;
    for i in 1..line.len() {
        dp[dp_i][0] = dp[dp_i - 1][0].max(dp[dp_i - 1][1]).max(dp[dp_i - 1][2]);
        let can_left_last0 = if line[i - 1].x >= (line[i].x - line[i].h) {
            0
        } else {
            1usize
        };
        let can_left_last1 = can_left_last0;
        let can_left_last2 = if (line[i - 1].x + line[i - 1].h) >= (line[i].x - line[i].h) {
            0
        } else {
            1usize
        };
        dp[dp_i][1] = (dp[dp_i - 1][0] + can_left_last0)
            .max(dp[dp_i - 1][1] + can_left_last1)
            .max(dp[dp_i - 1][2] + can_left_last2);
        let can_right = if i < (line.len()) - 1 && (line[i].x + line[i].h) < line[i + 1].x {
            1usize
        } else {
            0
        };
        dp[dp_i][2] = dp[dp_i - 1][0].max(dp[dp_i - 1][1]).max(dp[dp_i - 1][2]) + can_right;
        dp_i += 1;
    }
    dp[dp_i - 1][2] += 1; // we can always do this with the last tree
    println!(
        "{}",
        dp[dp_i - 1][0].max(dp[dp_i - 1][1]).max(dp[dp_i - 1][2])
    );
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
