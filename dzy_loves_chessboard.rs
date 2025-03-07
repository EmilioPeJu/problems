use std::collections::VecDeque;
use std::io;

struct Scan {
    buffer: VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
        }
    }

    fn next_line(&self) -> io::Result<String> {
        let mut line = String::new();
        match io::stdin().read_line(&mut line)? {
            0 => Err(io::Error::new(io::ErrorKind::Other, "EOF")),
            _ => Ok(line),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                match token.parse() {
                    Ok(x) => {
                        return x;
                    }
                    _ => {
                        panic!("parse");
                    }
                }
            }
            let line = self.next_line().unwrap();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn dfs(board: &mut Vec<Vec<char>>, letter: char, i: usize, j: usize) {
    board[i][j] = letter;
    for (new_i, new_j) in [
        (i + 1, j),
        (i.saturating_sub(1), j),
        (i, j + 1),
        (i, j.saturating_sub(1)),
    ] {
        if new_i == i && new_j == j {
            continue;
        }
        if new_i >= board.len() || new_j >= board[0].len() {
            continue;
        }
        if board[new_i][new_j] == '.' {
            dfs(board, if letter == 'W' { 'B' } else { 'W' }, new_i, new_j);
        }
    }
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut board: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let line: Vec<char> = scan.next::<String>().chars().take(m).collect();
        board.push(line);
    }
    for i in 0..n {
        for j in 0..m {
            if board[i][j] == '.' {
                dfs(&mut board, 'B', i, j);
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            print!("{}", board[i][j]);
        }
        println!();
    }
    Ok(())
}
