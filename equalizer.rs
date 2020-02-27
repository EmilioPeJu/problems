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
    let mut arr1: Vec<u8> = scan
        .next_line()
        .chars()
        .take(n)
        .map(|x| if x == '0' { 0 } else { 1 })
        .collect();
    let arr2: Vec<u8> = scan
        .next_line()
        .chars()
        .take(n)
        .map(|x| if x == '0' { 0 } else { 1 })
        .collect();
    if n == 1 {
        println!("{}", if arr1[0] == arr2[0] { 0 } else { 1 });
        return;
    }
    let mut result = 0usize;
    for i in 1..arr1.len() {
        if arr1[i] != arr2[i] && arr1[i - 1] != arr2[i - 1] && arr1[i] != arr1[i - 1] {
            arr1.swap(i, i - 1);
            result += 1;
        }
    }
    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] {
            result += 1;
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
