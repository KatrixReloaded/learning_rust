fn main() {
    // stacks are stored in stack frames, store only static data as stack frame size is fixed
    // heaps store dynamic data and we store the reference to that data in our vars
    // functions can be declared inside another function
    a();
    fn b() {
        // here, world is stored dynamically, it's in the heap, 
        // x is a reference to world on the heap
        let _x: String = String::from("world"); // stack frame 2: [b()     x]
    }
    b();

    ownership_main();
}

fn a() {
    // hello is statically typed in the code, stored in binary, x stores a reference to that
    let _x: &str = "hello";
    let _y: i32 = 22; // stack frame 1: [a()     x y]
}

fn ownership_main() {
    // ================ Ownership Rules ================
    // 1. Each value in Rust has a variable that's called it's owner 
    // 2. There can only be one owner at a time 
    // 3. When the owner goes out of scope, the value will be dropped
    { // s is not valid here
        let _s: String = String::from("hello"); // s is valid from this point forward
        // do things with s
    } // s is no longer valid as scope has ended, the value is deallocated from the heap 
    
    let x: i32 = 5;
    let _y: i32 = x; // x is copied into y, so y is also 5; Copy

    let s1: String = String::from("hello");
    let s2: String = s1; // s1 is NOT copied into s2, the value is moved to s2
    // s1 no longer owns that value, if I try to println s1,
    // I'll get an error; Move 

    let s3: String = s2.clone(); // creates a copy instead of moving, more expensive
    println!("{s2}");
    println!("{s3}");

    let s: String = String::from("hello");
    transfer_ownership(s);
    // printing s in this scope now would throw an error
    // println!("{s}");
    // NOTE: Type String does not have a Copy trait, so it is moved
    // However, if I tried doing the same thing with a primitive datatype,
    // it would work fine as it creates a copy by default instead of moving


    // the reverse of what we did above, the function that we call gives s the ownership
    let s: String = gives_ownership();
    println!("{s}");

    // this function that we call takes the ownership of the value and gives it back to s
    let s: String = takes_and_gives_back(s);
    println!("{s}");


    // ================ Rules of References ================
    // 1. At any given time, there can be one mutable reference or any number of immutable references
    // 2. References must always be valid
    // Suppose I call a function, it declares a variable in it and tries to return a reference to
    // that variable, I'll get an error as we're trying to borrow a value that does not exist

    // If we don't want to transfer ownership, we pass a reference
    let length: usize = calc_len(&s);
    println!("Length of {s} is {length}");

    // to make a reference mutable
    // mutable references have restrictions
    // If I try to pass two mutable references of the same variable in the same scope, I get an
    // error
    // multiple immutable references are fine, however,
    // if an immutable reference exists in a scope, I cannot create a mutable reference
    // because Rust doesn't expect an immutable reference to have a different value at runtime
    let mut s: String = String::from("hello");
    let length: usize = calc_len2(&mut s);
    println!("Length of {s} is {length}");
}

fn transfer_ownership(some_string: String) {
    println!("{some_string}");
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calc_len(s1: &String) -> usize {
    // s1 is a pointer that points to s, which in turn points to the value "hello"
    // references are immutable by default, if I try to change the value of s1, I get an error
    let length = s1.len();
    length
}


fn calc_len2(s1: &mut String) -> usize {
    s1.push_str(", world");
    let length = s1.len();
    length
}
