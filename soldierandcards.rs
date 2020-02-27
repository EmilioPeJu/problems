use std::collections::HashSet;
use std::collections::VecDeque;

struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> VecDeque<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn get_state(card1: &VecDeque<usize>, card2: &VecDeque<usize>) -> (u64, u64) {
    let mut r1 = 0u64;
    let mut r2 = 0u64;
    for item in card1.iter() {
        r1 <<= 4;
        r1 |= 1 << *item as u64;
    }
    for item in card2.iter() {
        r2 <<= 4;
        r2 |= 1 << *item as u64;
    }
    return (r1, r2);
}

fn _main() {
    let mut scan = Scan::new();
    // maximum number of states is (n+1)!,
    // that means that another way of solving this
    // is breaking the loop when number of wars is
    // around 40 millions in which case, it's a loop
    let mut visited = HashSet::new();
    let _n: usize = scan.next();
    let k1: usize = scan.next();
    let mut card1: VecDeque<usize> = scan.next_n(k1);
    let k2: usize = scan.next();
    let mut card2: VecDeque<usize> = scan.next_n(k2);
    let mut wars = 0usize;
    while !card1.is_empty() && !card2.is_empty() {
        let first = card1.pop_front().unwrap();
        let second = card2.pop_front().unwrap();
        if first > second {
            card1.push_back(second);
            card1.push_back(first);
        } else {
            card2.push_back(first);
            card2.push_back(second);
        }
        wars += 1;
        let state = get_state(&card1, &card2);
        if visited.contains(&state) {
            break;
        }
        visited.insert(state);
    }
    if !card1.is_empty() && !card2.is_empty() {
        println!("-1");
    } else {
        println!("{} {}", wars, if card1.is_empty() { 2 } else { 1 });
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
