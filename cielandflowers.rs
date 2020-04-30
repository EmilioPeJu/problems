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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn process(colors: &[u64], perm: &Vec<usize>) -> u64 {
    let mut result = 0u64;
    let mut lcolors = [0u64, 0, 0];
    lcolors.clone_from_slice(colors);
    for current in perm {
        if *current == 3 {
            let take = *lcolors.iter().min().unwrap();
            result += take;
            for i in &mut lcolors {
                *i -= take;
            }
            continue;
        }
        result += lcolors[*current] / 3;
        lcolors[*current] %= 3;
    }
    result
}

fn permutations(colors: &[u64], current: &mut Vec<usize>, step: usize, result: &mut u64) {
    if step == 4 {
        *result = (*result).max(process(colors, current));
        return;
    }
    for i in 0..4 {
        if let None = current.iter().find(|x| **x == i) {
            current.push(i);
            permutations(colors, current, step + 1, result);
            current.pop();
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let colors: Vec<u64> = scan.next_n(3);
    let mut result = 0u64;
    let mut perm: Vec<usize> = vec![];
    // this approach is shit, the simple approach is
    // the best of choosing 0, 1 and 2 mixing bouquet
    permutations(&colors, &mut perm, 0, &mut result);
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
