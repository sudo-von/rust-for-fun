fn main() {
    print_phase("Print my argument!");
}

fn print_phase(phrase: &str) {
    println!("Hello from the function! {}", phrase);

    println!("{}", gcd(20, 5));

    println!("{}", multiple_return(false));
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn multiple_return(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}
