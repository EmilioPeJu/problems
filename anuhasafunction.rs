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
    // Important to note that f(a,b) = a & ~b
    //
    // A better alternative, calculate the prefix and suffix
    // array for ~(a_i) & ~(a_{i+1})
    // for each element calculate = a[i] &  P[i-1] & S[i + 1] and
    // choose the max element, that must be the first element in
    // result, and then the rest (order doesn't matter)
    let mut scan = Scan::new();
    let n: usize = scan.next();
    // shit, I should've used a vector instead
    let mut bits: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let arr: Vec<usize> = scan.next_n(n);
    let mut arr2: Vec<usize> = arr.iter().cloned().collect();
    for &item in arr.iter() {
        let mut pow2 = 1usize;
        for i in 0..32 {
            if item & pow2 != 0 {
                *bits.entry(i).or_insert(0) += 1;
            }
            pow2 *= 2;
        }
    }
    for item in arr2.iter_mut() {
        let mut pow2 = 1usize;
        for i in 0..32 {
            if *item & pow2 != 0 {
                if *bits.get(&i).unwrap() > 1 {
                    *item = *item & !pow2;
                }
            }
            pow2 *= 2;
        }
    }
    let max = arr2.iter().max().unwrap();
    let index = arr2.iter().position(|&x| x == *max).unwrap();
    print!("{} ", arr[index]);
    for i in 0..arr.len() {
        if i != index {
            print!("{} ", arr[i]);
        }
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
