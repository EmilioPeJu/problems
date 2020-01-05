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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let arr: Vec<usize> = scan.next_n(n);
    // [activity][day];
    // [activity] = 0 is contest
    //              1 is gym
    // [day] = number of days considered
    // the 'does nothing' option is included in both activities, so we
    // don't need to consider it explicitly
    let mut table = [[0usize; 2]; 128];
    let mut table_index = 1usize;
    for &item in arr.iter() {
        let is_contest = if item & 1 != 0 { 1 } else { 0 };
        let is_gym = if item & 2 != 0 { 1 } else { 0 };
        table[table_index][0] =
            (table[table_index - 1][0]).max(table[table_index - 1][1] + is_contest);
        table[table_index][1] = (table[table_index - 1][0] + is_gym).max(table[table_index - 1][1]);
        table_index += 1;
    }
    // number of days that he does something
    let result = table[table_index - 1][0].max(table[table_index - 1][1]);
    // number of days that he rests
    println!("{}", arr.len() - result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
