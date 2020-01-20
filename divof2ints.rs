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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn find_div(x: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 1..=x {
        if x % i == 0 {
            result.push(i);
        }
    }
    return result;
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut arr: Vec<usize> = scan.next_n(n);
    let x = *arr.iter().max().unwrap();
    let div_x = find_div(x);
    for &item in div_x.iter() {
        *arr.iter_mut().find(|x| **x == item).unwrap() = 0;
    }
    let y = *arr.iter().max().unwrap();
    println!("{} {}", x, y);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
