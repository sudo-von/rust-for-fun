fn main() {
    let name = String::from("Tyler");

    let course = "Rust".to_string();

    let new_name = name.replace("Tyler", "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    let str1 = "Hello";
    println!("{}", str1);

    let str2 = "hello";
    let str3 = str2.to_string();
    let str4 = &str3;

    println!("{}", str2);
    println!("{}", str3);
    println!("{}", str4);
}
