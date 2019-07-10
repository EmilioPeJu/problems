fn is_vowel(c: char) -> bool {
    return c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U' || c == 'Y';
}
fn _main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Fail to read");
    let mut result = 0usize;
    let mut index = 0usize;
    for c in line.trim().chars() {
        index += 1;
        if is_vowel(c) {
            result = result.max(index);
            index = 0;
        }
    }
    index += 1;
    result = result.max(index);
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
