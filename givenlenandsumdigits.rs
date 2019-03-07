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

}

fn _main() {
    let mut scan = Scan::new();
    let m: isize = scan.next();
    let s: isize = scan.next();
    let nnines = s / 9;
    let last = s % 9;
    if nnines > m || nnines == m && last != 0 || m>1 && s == 0 {
        println!("-1 -1");
    } else if m == 1 && s == 0 {
        println!("0 0");
    } else if m == 1 {
        println!("{} {}", s, s);
    } else if nnines == m {
        let sol = format!("{}", "9".repeat(nnines as usize));
        println!("{} {}", sol, sol);
    } else {
        let mut result = String::new();
        let mut s1 = s;
        for i in 1..=m {
            for d in 0..=9 {
                if i==1 && d==0 { continue; }
                if s1-d <= 9*(m-i) && s1-d >= 0 {
                    result = format!("{}{}", result, d);
                    s1 -= d;
                    break;
                }
            }
        }
        print!("{}", result);
        result.clear();
        s1 = s;
        for i in 1..=m {
            for d in (0..=9).rev() {
                if  s1-d <= 9*(m-i) && s1-d >= 0 {
                    result = format!("{}{}", result, d);
                    s1 -= d;
                    break;
                }
            }
        }
        println!(" {}", result);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}

