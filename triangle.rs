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

fn is_triangle(a: usize, b: usize, c: usize) -> bool {
    // parameters are passed sorted in increasing order
    return a + b > c;
}

fn is_collinear(a: usize, b: usize, c: usize) -> bool {
    // parameters are passed sorted in increasing order
    return a + b == c;
}

fn _main() {
    let mut scan = Scan::new();
    let mut arr: Vec<usize> = scan.next_n(4);
    arr.sort();
    if is_triangle(arr[0], arr[1], arr[2])
        || is_triangle(arr[0], arr[1], arr[3])
        || is_triangle(arr[0], arr[2], arr[3])
        || is_triangle(arr[1], arr[2], arr[3])
    {
        println!("TRIANGLE");
    } else if is_collinear(arr[0], arr[1], arr[2])
        || is_collinear(arr[0], arr[1], arr[3])
        || is_collinear(arr[0], arr[2], arr[3])
        || is_collinear(arr[1], arr[2], arr[3])
    {
        println!("SEGMENT");
    } else {
        println!("IMPOSSIBLE");
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
