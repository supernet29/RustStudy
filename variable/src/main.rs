// This is constant value... it needs to specifing type.
const MAX_POINTS: u32 = 100_000;

fn main() {
    // if there is no mut keyword, it will occur compile error.
    // because basically variable is immutable.
    let mut x = 5;
    println!("The value of x is {}.", x);

    x += 2;
    println!("The value of x is {}.", x);

    println!("The value of MAX_POINTS is {}.", MAX_POINTS);
}
