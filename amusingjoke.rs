
fn get_input() -> Vec<char> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().chars().collect()
}

fn main() {
    let mut a = get_input();
    for c in get_input() { a.push(c); }
    let mut c = get_input();
    a.sort();
    c.sort();
    if a == c { println!("YES"); }
    else      { println!("NO"); }
}
