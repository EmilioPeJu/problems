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
    let mut result: usize = 0;
    for _ in 0..n {
        let input: String = scan.next();
        if input == "Tetrahedron" { result += 4; }
        else if input == "Cube" { result += 6; }
        else if input == "Octahedron" { result += 8; }
        else if input == "Dodecahedron" { result += 12; }
        else if input == "Icosahedron" { result += 20; }
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
