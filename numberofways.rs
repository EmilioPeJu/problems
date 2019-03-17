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
    let mut arr: Vec<i64> = scan.next_n(n);
    // calc accumulative sums
    for index in 1..n {
        arr[index] += arr[index-1];
    }
    let mut result = 0u64;
    let total = arr.last().unwrap();
    if total % 3 != 0 {
        println!("0");
        return;
    }
    let mut first: Vec<usize> = vec![];
    let mut second: Vec<usize> = vec![];
    for (index, &sum) in arr.iter().take(n-1).enumerate() {
        if sum == total/3 {
            first.push(index);
        }
        if sum == 2*total/3 {
            second.push(index);
        }
    }
    let mut index1 = 0;
    let mut index2 = 0;
    while index1 < first.len() && index2 < second.len() {
        if first[index1] < second[index2] {
            result += second.len() as u64 - index2 as u64;
            index1 += 1
        } else {
            index2 += 1
        }
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
