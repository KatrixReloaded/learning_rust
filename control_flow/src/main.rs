fn main() {
    let num = 4;

    if num < 5 {
        println!("statement is true");
    } else {
        println!("statement is false");
    }

    // if condition in variable
    let num = if num<5 {5} else {3};
    println!("num: {0}", num);

    // returning values from loop
    let mut counter = 0;
    let num = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The loop result is: {num}");

    // loop labels to disambiguate between multiple loops
    counter = 0;

    'outer_loop: loop {
        println!("count: {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("count end value: {counter}");

    // conditional loop with while
    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("LIFTOFF!");

    // same loop as above in for 
    // NOTE: rev() is used to reverse the values. Also, (1..4) is exclusive of the end value,
    // whereas, (1..=4) is used for inclusive on both ends
    for a in (1..4).rev() {
        println!("{a}");
    }
    println!("LIFTOFF!");

    // looping through a collection with for 
    let num = [1,2,3,4,5];
    
    for element in num {
        println!("Value in num is: {element}");
    }
}
