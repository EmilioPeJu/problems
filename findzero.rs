use std::io;

fn fun(x: f64) -> f64 {
    (x * x) as f64 - 4.0
}

fn _main() -> io::Result<()> {
    let mut a = 0.0f64;
    let mut b = 5.0f64;
    let mut sol = a;
    while fun(sol).abs() > 0.01 {
        if fun(sol) <= 0.0 {
            a = sol;
        } else {
            b = sol;
        }
        sol = (a + b) / 2.0;
        println!("a = {} b = {}", a, b);
        println!("sol = {}", sol);
        println!("f(sol) = {}", fun(sol));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap()?;
    Ok(())
}
