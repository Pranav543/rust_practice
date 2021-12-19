fn sqr(x: f64) -> f64 {
    return x * x;
    // Another way
    // x * x
    // No Semicolon Needed!
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
}