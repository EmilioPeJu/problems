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

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut arr: Vec<(usize, usize)> = vec![];
    let mut result: Vec<usize> = vec![];
    for i in 1..=n {
        let current: usize = scan.next();
        arr.push((current, i));
    }
    arr.sort();
    result.push(arr[0].1);
    for i in 1..n {
        if arr[i].0 != arr[i - 1].0 {
            result.push(arr[i].1);
            if result.len() == m {
                break;
            }
        }
    }
    if result.len() < m {
        println!("NO");
    } else {
        println!("YES");
        for &item in result.iter() {
            print!("{} ", item);
        }
        println!();
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
