fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    let mut number = 3;
    while number != 0 {
        println!("number: {}", number);
        number -= 1;
    }
    println!("liftoff!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for a in (1..4).rev() {
        println!("{}!", a)
    }
    println!("liftoff!!!");
}
