use std::collections::HashSet;

fn get_line() -> String {
    let mut line = String::new();
    loop {
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => { break line; },
            Err(_) => continue
        }
    }
}

fn get_letters() -> HashSet<char> {
    get_line().chars().filter(|&x| char::is_alphabetic(x)).collect()
}

fn main() {
    let input = get_letters();
    println!("{:?}", input.len());
}
