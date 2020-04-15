struct Scan {}

impl Scan {
    fn new() -> Scan {
        Scan {}
    }

    fn next_line(&self) -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Fail to read");
        line
    }
}

fn sum_of_digits(x: u32) -> u32 {
    let mut result = 0u32;
    let mut tmp = x;
    while tmp > 0 {
        result += tmp % 10;
        tmp /= 10;
    }
    result
}

fn _main() {
    let scan = Scan::new();
    let line = scan.next_line();
    let mut result = 1usize;
    let mut acc = 0u32;
    let mut ndigits = 0usize;
    for c in line.chars() {
        if let Some(d) = c.to_digit(10) {
            acc += d;
            ndigits += 1;
        }
    }
    if ndigits == 1 {
        println!("0");
        return;
    }
    while acc >= 10 {
        acc = sum_of_digits(acc);
        result += 1;
    }
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
