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

struct Point { x: isize, y: isize, }

impl Point {
    fn is_valid(&self) -> bool {
        self.x >= -1000 && self.x <= 1000 && self.y >= -1000 && self.y <= 1000
    }
}

fn _main() {
    let mut scan = Scan::new();
    let p1: Point = Point { x: scan.next(), y: scan.next() };
    let p2: Point = Point { x: scan.next(), y: scan.next() };
    let v = Point {x: p2.x - p1.x, y: p2.y - p1.y};
    if v.x*v.y != 0 && v.x.abs() != v.y.abs() {
        println!("-1");
        return;
    }
    if v.x == 0  || v.y == 0 {
        // not really necesary, the input range is 10 times smaller
        // that means, no matter in which direction you form the square, 
        // it will be included in the output range
        for &sign in [1, -1].iter() {
            let p3 = Point {x: p1.x + sign*v.y,  y: p1.y + sign*v.x};
            let p4 = Point {x: p2.x + sign*v.y, y: p2.y + sign*v.x};
            if p3.is_valid() && p4.is_valid() {
                println!("{} {} {} {}", p3.x, p3.y, p4.x, p4.y);
                return;
            }
        }
        println!("-1");
    } else {
        println!("{} {} {} {}", p1.x + v.x, p1.y, p1.x, p1.y + v.y);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
