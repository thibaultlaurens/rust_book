// constant definition
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("{}", MAX_POINTS);

    // mut keyword
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}", y);

    // integer overflow (and complement wrapping in release build)
    // let z: u8 = 255;
    // let z = z + 1;
    // println!("The value of z is {}", z);

    // tuple and pattern matching
    let tup: (i32, f64, u8) = (250, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x y z is {} {} {}", x, y, z);

    // tuple and indexes
    let tup2 = (251, 7.4, 2);
    let x = tup2.0;
    let y = tup2.1;
    let z = tup2.2;
    println!("The value of x y z is {} {} {}", x, y, z);

    // arrays and vectors
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // function call
    another_function();
}
