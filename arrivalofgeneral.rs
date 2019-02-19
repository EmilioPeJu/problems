use std::collections::VecDeque;
use std::str::FromStr;

struct InputScanner {
    buffer: VecDeque<String>
}

impl InputScanner {

    fn new() -> InputScanner {
        InputScanner{ buffer: VecDeque::new() }
    }

    fn next<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            self.buffer = match std::io::stdin().read_line(&mut line) {
                Ok(_) => line.split_whitespace().map(String::from).collect(),
                Err(_) => continue,
            }
        }
        self.buffer.pop_front().unwrap().parse().ok().unwrap()
    }

    fn read_n<T: FromStr>(&mut self, n : usize)->Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

}

fn get_max_min<T : std::cmp::PartialOrd>(arr : &Vec<T>) -> (usize, usize) {
    let mut i_min : usize = 0;
    let mut i_max : usize = 0;
    for index in 0..arr.len() {
        if arr[index] > arr[i_max] {
            i_max = index;
        }
        if arr[index] <= arr[i_min] {
            i_min = index;
        }
    }
    (i_max, i_min)
}

fn main() {
    let mut scan = InputScanner::new();
    let n : usize = scan.next();
    let arr : Vec<usize> = scan.read_n(n);
    let (i_max, i_min) = get_max_min(&arr);
    let result : usize = i_max + (n - i_min -1) - 
        (if i_max > i_min { 1 } else { 0 });
    println!("{}", result);
}
