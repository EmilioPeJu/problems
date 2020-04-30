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

fn _main() {
    let mut scan = Scan::new();
    let input: Vec<char> = scan.next::<String>().chars().collect();
    let mut output: Vec<char> = Vec::new();
    if input.len() < 3 {
        let output: String = input.into_iter().collect();
        println!("{}", output);
        return;
    }
    output.push(input[0]);
    output.push(input[1]);
    let mut o_i = 1;
    for i in 2..input.len() {
        let tripple_typo = input[i] == input[i - 1] && input[i] == input[i - 2];
        let double_typo = o_i >= 2 && input[i] == output[o_i] && output[o_i - 1] == output[o_i - 2];
        if tripple_typo || double_typo {
            continue;
        }
        output.push(input[i]);
        o_i += 1;
    }
    let output: String = output.into_iter().collect();
    println!("{}", output);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
