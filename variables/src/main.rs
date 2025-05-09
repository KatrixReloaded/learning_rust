fn main() {
    // mut variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // signed integers
    let x: i32 = -5;
    println!("The value of x is: {x}");

    // unsigned integers
    let x: u8 = 234;
    println!("The value of x is: {x}");

    // floating point; only f32 and f64
    let x: f32 = 2.0;
    println!("The value of x is: {x}");

    // boolean
    let x: bool = true;
    println!("The value of x is: {x}");

    // tuple
    access_tuple();
    destructuring_tuple();
    mutable_tuple();

    // character
    let x: char = 'a';
    println!("The value of x is: {x}");

    // array
    arrays([1,2,3,4,5]);

    var_blocks();
    shadowing();
    i32 = fun_return();
}

fn shadowing() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");
}

fn destructuring_tuple() {
    let tup = (250, 2.0, 18);
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}

fn access_tuple() {
    let tup = (250, 2.0, 18);

    println!("The value of x is: {0}", tup.0);
    println!("The value of y is: {0}", tup.1);
    println!("The value of z is: {0}", tup.2);
}

fn mutable_tuple() {
    let mut tup: (u64, i32) = (256, -127);
    tup.0 = 127;
    tup.1 += 127;

    println!("The value of tup is: {0}, {1}", tup.0, tup.1);
}

// also an example of how to define params
fn arrays(_arr: [i32;5]) {
    let a = [1,2,3,4,5];
    let b: [i32; 5] = [1,2,3,4,5];
    // c = [3,3,3,3,3]
    let c = [3; 5];

    println!("Array: a, index: 1 => {0}", a[1]);
    println!("Array: b, index: 1 => {0}", b[1]);
    println!("Array: c, index: 1 => {0}", c[1]);
}

fn var_blocks() {
    let y = {
        let x = 5;
        x += 1;
    };

    println!("Value of y is: {}", y);
}

fn fun_return() -> i32 {
    5
}
