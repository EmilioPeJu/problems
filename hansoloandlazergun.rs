use std::collections::HashSet;

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
}

fn gdc(a: isize, b: isize) -> isize {
    let (mut a, mut b) = (a.min(b), a.max(b));
    while a > 0 {
        let aux = a;
        a = b % a;
        b = aux;
    }
    if b == 0 {
        1
    } else {
        b
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Rational {
    a: isize,
    b: isize,
}

fn _main() -> std::io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let x0: isize = scan.next();
    let y0: isize = scan.next();
    let mut curves: HashSet<Rational> = HashSet::new();
    for _ in 0..n {
        let x: isize = scan.next();
        let y: isize = scan.next();
        let mut xx0 = (x - x0) * if (y - y0) == 0 { 1 } else { (y - y0).signum() };
        let mut yy0 = (y - y0).abs();
        if xx0 == 0 {
            yy0 = 1;
        }
        if yy0 == 0 {
            xx0 = 1;
        }
        let c = gdc(xx0.abs(), yy0);
        let frac = Rational {
            a: yy0 / c,
            b: xx0 / c,
        };
        curves.insert(frac);
    }
    println!("{}", curves.len());
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
