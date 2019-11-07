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

fn print_result(digits: &String, track: &[[State; 8]; 128], i: usize, j: usize) -> () {
    match track[i][j] {
        State {
            parentj: _,
            action: Action::TAKE,
        } => {
            print!("{}", digits.as_bytes()[i - 1] - 48);
        }
        State {
            parentj,
            action: Action::DONTTAKE,
        } => {
            print_result(digits, track, i - 1, parentj);
        }
        State {
            parentj,
            action: Action::ADDANDTAKE,
        } => {
            print_result(digits, track, i - 1, parentj);
            print!("{}", digits.as_bytes()[i - 1] - 48);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    TAKE,
    DONTTAKE,
    ADDANDTAKE,
}

#[derive(Debug, Copy, Clone)]
struct State {
    parentj: usize,
    action: Action,
}

fn _main() {
    let mut scan = Scan::new();
    // we use the property (a*b+c)%x = ((a%x)*b+c)%x
    // and a number e.g 876 can be expresed as (8*10+7)*10+6
    // and calculation can be propagated from most significant
    // part to less significan part: 876 % 8 = ((((8%8)*10+7)%8)+6)%8
    let digits: String = scan.next();
    let mut dp = [[false; 8]; 128];
    let mut track = [[State {
        parentj: 0,
        action: Action::TAKE,
    }; 8]; 128];
    let mut index = 1usize;
    for digit_ch in digits.chars() {
        let digit: usize = digit_ch.to_digit(10).unwrap() as usize;
        dp[index][digit % 8] = true; // current digit
        track[index][digit % 8] = State {
            parentj: 0, // not used
            action: Action::TAKE,
        };
        for j in 0..8 {
            if dp[index - 1][j] {
                dp[index][j] = true; // skip current digit
                track[index][j] = State {
                    parentj: j,
                    action: Action::DONTTAKE,
                };
                dp[index][(j * 10 + digit) % 8] = true; // add current to last state
                track[index][(j * 10 + digit) % 8] = State {
                    parentj: j,
                    action: Action::ADDANDTAKE,
                };
            }
        }
        index += 1;
    }
    if dp[index - 1][0] {
        println!("YES");
        print_result(&digits, &track, index - 1, 0);
        println!();
    } else {
        println!("NO");
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
