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
#[derive(Eq, Ord, PartialOrd, PartialEq)]
struct Exam {
    a: usize,
    b: usize,
}
fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut arr: Vec<Exam> = vec![];
    for _ in 0..n {
        let a: usize = scan.next();
        let b: usize = scan.next();
        arr.push(Exam { a: a, b: b });
    }
    arr.sort();
    let mut result = arr[0].a.min(arr[0].b);
    for i in 1..n {
        let min_date = arr[i].a.min(arr[i].b);
        let max_date = arr[i].a.max(arr[i].b);
        if min_date >= result {
            result = min_date;
        } else {
            result = max_date;
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
