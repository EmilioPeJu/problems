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
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut assigned = [0usize; 100001];
    for _ in 0..m {
        let dance: Vec<usize> = scan.next_n(3);
        let mut used = [false; 4];
        used[0] = true;
        for i in dance.iter() {
            let color = assigned[*i];
            used[color] = true;
        }
        for i in dance.iter() {
            if assigned[*i] == 0 {
                let color: usize = used.iter().position(|x| *x == false).unwrap();
                assigned[*i] = color;
                used[color] = true;
            }
        }
    }
    for i in 1..=n {
        print!("{} ", if assigned[i] != 0 { assigned[i] } else { 1 });
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
