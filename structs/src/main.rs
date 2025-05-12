// we used String and not &str so that each instance of this struct owns all of its data
// and the data is valid as long as the struct is valid 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// tuple structs
#[derive(Debug)]
struct Colour(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Kartikeya"),
        email: String::from("kartikeya@hooli.com"),
        sign_in_count: 1
    };
    println!("{0}", user1.username);

    let _user = build_user_1(String::from("Akeno"), String::from("akeno@hooli.com"));

    let mut user1 = User {
        active: true,
        username: String::from("Kartikeya"),
        email: String::from("kartikeya@hooli.com"),
        sign_in_count: 1
    };
    user1.email = String::from("katrix@hooli.com");
    println!("{0}", user1.email);

    // soo we are moving the username data from user1 to user2, since it's of type String.
    // had we assigned new values to both Strings, user1 would be completely valid
    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("kartikeya@hooli.com"),
        sign_in_count: user1.sign_in_count
    };

    // struct update syntax
    //let user2 = User { 
    //    email: String::from("kartikeya@hooli.com"),
    //    ..user1
    //};

    // tuple structs
    let black = Colour(0,0,0);
    let origin = Point(0,0,0);
    println!("{:?}", black);
    dbg!(origin);

    // unit tuples and unit-like structs???
    // ()
    // struct User;
    
    borrow_fields();

    // example of using structs
    calc_rect_area();

    // Methods
    test_method();
    multiple_params_in_methods();
}

fn build_user_1(username: String, email: String) -> User {
    User {
        active: true,
        username: username, // can use param like this
        email, // OR like this, since the field name and param name are exactly the same
        sign_in_count: 1
    }
}

// borrowing fields of a struct 
fn borrow_fields() {
    struct Coord {
        x:i32,
        y:i32,
    }
    
    let mut c = Coord{x: 10, y: 20};
    // while c.x is borrowed, both p and p.x lose their permissions temporarily, not p.y
    let x = &mut c.x;

    *x += 1;

    println!("{x}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calc_rect_area() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // syntax for printing structs; will have to use this for tuples, tuple structs, arrays, etc.
    println!("The rectangle is {rect1:?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// ================ Methods Syntax ================
// methods are similar to functions except the fact that they are declared in the context of a
// struct or an enum or a trait object
// the first param is always self, which represents the instance of the struct the method is being
// called on
// We can create multiple impl blocks for the same struct. Why? idk... yet
impl Rectangle {
    // &self is actually short for self: &Self
    // NOTE: We use references when we only want to read the data and not have ownership of the
    // value, we don't want to write to it here so used &self, if writing, use &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // we can use the same name for a method as that of a field in the struct 
    // when calling, if we write `rect1.width`, Rust knows we're talking about the field
    // when we add parantheses to it, Rust knows we mean the method
    // kinda like polymorphism
    fn width(&self) -> bool {
        self.width > 0
    }

    // multiple params in a method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions are functions declared inside the impl block that are not methods 
    // you can differentiate between methods and associated functions by checking the params
    // methods have self as the first param, whereas, associated functions don't
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn test_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The width of the rectangle is greater than 0: {}", rect1.width());
}

fn multiple_params_in_methods() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    calling_associated_functions();
}

fn calling_associated_functions() {
    let sq1 = Rectangle::square(10);

    println!("The area of the square is: {}", sq1.area());

    multiple_ways_of_calling_methods_and_fns();
}

fn multiple_ways_of_calling_methods_and_fns() {
    let r1 = Rectangle {
        width: 1,
        height: 2,
    };

    let r2 = Rectangle {
        width: 3,
        height: 4,
    };

    // basically, we can call methods like we call associated functions too
    let area1 = r1.area();
    let area2 = Rectangle::area(&r1);
    assert_eq!(area1, area2);

    println!("Can r2 hold r1?: {}", r2.can_hold(&r1));
    println!("Can r2 hold r1?: {}", Rectangle::can_hold(&r2, &r1));
}
