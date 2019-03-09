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
    let arr: Vec<isize> = scan.next_n(n);
    let allones: usize = arr.iter().filter(|&&x| x==1).count();
    let total: isize = arr.iter().sum();
    // convert to an array in which Kadane can be used
    let arr2: Vec<isize> = arr.into_iter().map(|x| if x==1 { -1 } else { 1 }).collect();
    let mut cnt: isize = 0;
    let mut maxcnt: isize = 0;
    if allones == n {
        println!("{}", total-1);
        return;
    }
    for i in 0..n {
        cnt += arr2[i];
        if cnt < 0 { cnt = 0; }
        maxcnt = maxcnt.max(cnt);
    }
    println!("{}", total + maxcnt);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
