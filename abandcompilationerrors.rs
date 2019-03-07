use std::collections::HashMap;
struct Scan {
    buffer: std::collections::VecDeque<String>
}

impl Scan {
    fn new() -> Scan {
        Scan { buffer: std::collections::VecDeque::new() }
    }

    fn next<T: std::str::FromStr>(&mut self)-> T {
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
    let n: usize = scan.next();
    let mut map: HashMap<usize, [usize; 3]> = HashMap::new();
    for _ in 0..n {
        let x: usize = scan.next::<usize>();
        map.entry(x).and_modify(|x| {(*x)[0]+=1;}).or_insert([1,0,0]);
    }
    for _ in 0..n-1 {
        let x = scan.next::<usize>();
        map.entry(x).and_modify(|x| {(*x)[1]+=1;});
    }
    for _ in 0..n-2 {
        let x = scan.next::<usize>();
        map.entry(x).and_modify(|x| {(*x)[2]+=1;});
    }
    let mut sol: [usize;2] = [0, 0];
    for (key, val) in map.iter() {
        if (*val)[0] != (*val)[1] {
            sol[0] = *key;
        }
        if (*val)[1] != (*val)[2] {
            sol[1] = *key;
        }
    }
    println!("{}\n{}", sol[0], sol[1]);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
