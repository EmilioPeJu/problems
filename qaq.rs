fn _main() {
    // crappy O(n^3) solution
    // it's better iterating A's and
    // use a Q accumulative counter array
    // to get how many there are before
    // and after
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("failed to read");
    let line: Vec<char> = line.chars().collect();
    let mut cnt = 0usize;
    for i in 0..line.len() {
        for j in i + 1..line.len() {
            for k in j + 1..line.len() {
                if line[i] == 'Q' && line[j] == 'A' && line[k] == 'Q' {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
