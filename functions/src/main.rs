fn main() {
    another_function(1, 2);

    let a = {
        let x = 3;
        x + 1
    };
    println!("The value of a is: {}", a);

    let b = five();
    println!("The value of b is: {}", b);

    let c = plus_one(5);
    println!("The value of c is: {}", c);
}

// function declaration
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
