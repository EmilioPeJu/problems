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
    let p: Vec<isize> = scan.next_n::<isize>(n)
        .into_iter().map(|x| if x==-1 {-1} else {x-1}).collect();
    let mut maxlevel: usize = 1;
    for i in 0..n {
        let mut current = i as isize;
        let mut level = 1;
        while p[current as usize] != -1 {
            current = p[current as usize];
            level += 1;
        }
        maxlevel = maxlevel.max(level);
    }
    println!("{}", maxlevel);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
