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

fn _main() -> std::io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        let arr: Vec<char> = scan.next::<String>().chars().collect();
        let n = arr.len();
        let mut pre1 = vec![0usize; n];
        let mut post1 = vec![0usize; n];
        pre1[0] = if arr[0] == '1' { 1 } else { 0 };
        for i in 1..n {
            pre1[i] += pre1[i - 1];
            if arr[i] == '1' {
                pre1[i] += 1;
            }
        }
        post1[n - 1] = if arr[n - 1] == '1' { 1 } else { 0 };
        for i in (0..(n - 1)).rev() {
            post1[i] += post1[i + 1];
            if arr[i] == '1' {
                post1[i] += 1;
            }
        }
        let mut result = pre1[n - 1].min(n - pre1[n - 1]);
        for i in 1..n {
            let lowerpart = pre1[i - 1].min(i - pre1[i - 1]);
            let upperpart = post1[i].min(n - i - post1[i]);
            result = result.min(lowerpart + upperpart);
        }
        println!("{}", result);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap()?;
    Ok(())
}
