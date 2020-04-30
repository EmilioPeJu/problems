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
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let mut counts = [0isize; 200002];
    let n: usize = scan.next();
    let q: usize = scan.next();
    let mut arr: Vec<u64> = scan.next_n(n);
    arr.sort();
    let mut arr2: Vec<u64> = vec![0u64; arr.len()];
    let mut inters: Vec<(usize, usize)> = Vec::new();
    for _ in 0..q {
        let mut l: usize = scan.next();
        let mut r: usize = scan.next();
        l -= 1;
        r -= 1;
        inters.push((l, r));
        // another way to do this is using a Fenwick tree
        counts[l] += 1;
        counts[r + 1] -= 1;
    }
    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }
    let mut ind: Vec<usize> = (0..arr.len()).collect();
    ind.sort_by_key(|x| counts[*x]);
    let mut arr_i = 0usize;
    for i in &ind {
        arr2[*i] = arr[arr_i];
        arr_i += 1;
    }
    let mut pre_arr2 = arr2.clone();
    for i in 1..pre_arr2.len() {
        pre_arr2[i] += pre_arr2[i - 1];
    }
    let mut result = 0u64;
    for i in &inters {
        result += pre_arr2[i.1];
        if i.0 > 0 {
            result -= pre_arr2[i.0 - 1];
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
