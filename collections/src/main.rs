fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    let mut v = vec![1,2,3];

    // updating a vector
    println!("{v:?}");

    // reading elements in a vector
    // this way would panic if out of bounds
    let third: &i32 = &v[2];
    println!("{third}");

    // this won't panic if out of bounds, it will return None
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("{third}"),
        None => println!("There is no third element"),
    }

    // iterating over vector
    for n_ref in &v {
        // n_ref is of type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    for n_ref in &mut v {
        *n_ref += 50;
        println!("{n_ref}");
    }

    // we can use enums to store multiple types in a vector
    #[derive(Debug)]
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Float(3.14),
        Spreadsheet::Text(String::from("01001000 01001001"))
    ];

    println!("{row:?}");
    
    // strings
    // new string 
    let _s = String::new();
    let mut s = String::from("foo");
    
    // appending, can use + too
    s.push_str("bar");
    println!("{s:?}");

    // + consumes ownership of a, while push_str does not 
    let a = String::from("foo");
    let b = String::from("bar");
    let s1 = a + &b;
    println!("{s1:?}");
    
    // push can be used to only add a single char
    s.push('l');
    println!("{s:?}");
    // Rust strings don't support indexing!
    // s[0] will revert
    // however, you can slice it instead of passing a single value
    let s_slice = &s[0..1];
    println!("{s_slice}");

    // methods for iterating over strings
    for c in s.chars() {
        println!("{c}");
    }
}
