struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
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
    let mut rooms = [false; 10];
    let n: usize = scan.next();
    let input: String = scan.next::<String>().chars().take(n).collect();
    for ch in input.chars() {
        if ch == 'L' {
            *rooms.iter_mut().find(|x| **x == false).unwrap() = true;
        } else if ch == 'R' {
            *rooms.iter_mut().rfind(|x| **x == false).unwrap() = true;
        } else {
            let digit: usize = ch as usize - 48;
            rooms[digit] = false;
        }
    }
    for elem in rooms.iter() {
        print!("{}", if *elem { 1 } else { 0 });
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
