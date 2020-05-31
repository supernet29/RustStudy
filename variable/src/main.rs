fn main() {
    // if there is no mut keyword, it will occur compile error.
    // because basically variable is constant.
    let mut x = 5;
    println!("The value of x is {}.", x);
    x = 6;
    println!("The value of x is {}.", x);
}
