fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    let mut y = 5;
    println!("The value of mut y is {}", y);

    y = 6;
    println!("The value of mut y is {}", y);

    const SECONDS: i8 = 60;
    println!("The value of SECONDS is {}", SECONDS);
}
