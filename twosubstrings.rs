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
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let mut i1: Vec<usize> = Vec::new();
    let mut i2: Vec<usize> = Vec::new();
    for i in 0..input.len()-1 {
        if input[i] == 'A' && input[i+1] == 'B' { i1.push(i); }
        if input[i] == 'B' && input[i+1] == 'A' { i2.push(i); }
    }
    if i1.len() == 0 || i2.len() == 0 {
        println!("NO");
        return;
    }
    let &firstab = i1.first().unwrap();
    let &firstba = i2.first().unwrap();
    let &lastab = i1.last().unwrap();
    let &lastba = i2.last().unwrap();
    if firstab+2 <= lastba || firstba+2 <= lastab {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
