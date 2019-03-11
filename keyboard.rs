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
    let keyboard: Vec<char> = "qwertyuiopasdfghjkl;zxcvbnm,./".chars()
        .collect();
    let dir: char = scan.next();
    let input: String = scan.next::<String>();
    let offset: isize = if dir == 'R' { -1 } else { 1 };
    for c in input.chars() {
        let mut index = keyboard.iter().position(|&x| x==c).unwrap();
        print!("{}", keyboard[(index as isize + offset) as usize]);
    }

}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
