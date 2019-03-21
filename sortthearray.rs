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
    let mut arr: Vec<isize> = scan.next_n(n);
    let mut first = std::usize::MAX;
    let mut second = n-1;
    for i in 1..n {
        if arr[i] < arr[i-1] {
            first = i - 1;
            break;
        }
    }
    for i in (1..n).rev() {
        if arr[i] < arr[i-1] {
            second = i;
            break;
        }
    }
    if first == std::usize::MAX {
        println!("yes\n1 1");
        return;
    }
    arr[first..second+1].reverse();
    let mut sorted = true;
    for i in  1..n {
        if arr[i] < arr[i-1] {
            sorted = false;
            break;
        }
    }
    if !sorted {
        println!("no");
    } else {
        println!("yes\n{} {}", first + 1, second + 1);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
