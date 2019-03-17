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
    let n: usize = scan.next();
    let mut found = false;
    let mut output: Vec<String> = vec![];
    for _ in 0..n {
        let mut line: String = scan.next();
        if !found {
            let mut line: Vec<char> = line.chars().collect();
            if line[0] == 'O' && line[1] == 'O' {
                found = true;
                line[0] = '+';
                line[1] = '+';
            } else if line[3] == 'O' && line[4] == 'O' {
                found = true;
                line[3] = '+';
                line[4] = '+';
            }
            output.push(line.iter().collect::<String>());
        } else { output.push(line); }

    }
    if found {
        println!("YES");
        for line in output.iter() {
            println!("{}", line);
        }
    } else { println!("NO"); }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
