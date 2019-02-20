use std::collections::HashSet;

fn get_int() -> usize {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<usize>().ok().unwrap()
}

fn get_input(limit: usize) -> HashSet<char> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.chars()
        .take(limit)
        .map(|x| x.to_ascii_lowercase())
        .filter(|x| x.is_alphabetic())
        .collect()
}

fn main() {
    let n = get_int();
    let input = get_input(n);
    println!("{}", if input.len() == 26 { "YES" } else { "NO" });
}
