fn main() {
    let number = 1;

    if number > 0 {
        println!("True");
    } else {
        println!("False");
    }

    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;
        loop {
            println!("Decreasing: {}", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

    let mut num2 = 0;

    while num2 < 5 {
        println!("Num: {}", num2);
        num2 += 1;
    }

    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }

    /*
    loop {
        println!("Infinite loop!")
    }
    */
}
