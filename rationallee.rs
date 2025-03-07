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
        let n: usize = scan.next();
        let k: usize = scan.next();
        let mut a: Vec<isize> = scan.next_n(n);
        let mut w: Vec<usize> = scan.next_n(k);
        let mut maxa: Vec<isize> = vec![std::isize::MIN; k];
        let mut mina: Vec<isize> = vec![std::isize::MAX; k];
        a.sort();
        a.reverse();
        w.sort();
        let mut i = 0usize;
        let mut j = 0usize;
        // share the maximum values among all
        while i < k {
            maxa[i] = maxa[i].max(a[i]);
            mina[i] = mina[i].min(a[i]);
            w[i] -= 1;
            i += 1;
        }
        // the rest should try to fill the smallest w first to get
        // bigger minimum element
        while i < n {
            while w[j] == 0 {
                j = (j + 1) % k;
            }
            mina[j] = mina[j].min(a[i]);
            maxa[j] = maxa[j].max(a[i]);
            w[j] -= 1;
            i += 1;
        }
        let result: i64 = maxa.into_iter().map(|x| x as i64).sum::<i64>()
            + mina.into_iter().map(|x| x as i64).sum::<i64>();
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
