fn main() {
    let tup = (500, "hi", true);

    println!("{}", tup.0);

    let (x, y, z) = tup;

    println!("x: {} y: {} z: {}", x, y, z);
}
