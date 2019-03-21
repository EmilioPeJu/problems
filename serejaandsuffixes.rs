use std::collections::HashSet;
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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let arr: Vec<usize> = scan.next_n(n);
    let mut cnt_arr = vec![0usize;n];
    let mut cnt = 0usize;
    let mut set: HashSet<usize>  = HashSet::new();
    for index in (0..n).rev() {
        let num = arr[index];
        if !set.contains(&num) {
            set.insert(num);
            cnt += 1;
        }
        cnt_arr[index] = cnt;
    }
    // queries
    for _ in 0..m {
        let q = scan.next::<usize>() - 1;
        let result = cnt_arr[q];
        println!("{}", result);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
