fn main() {
    println!("Hello, world!");

    another_function();
    parameter_function(32);
    let x = five();
    println!("{}", x);
    let y = plus_one(5);
    println!("{}", y);

}

fn another_function() {
    println!("Another function");
}

fn parameter_function(x: i32) {
    println!("The value of x is: {x}");
}

// Statements are instructions that perform some action and do not return a value
// Expressions evaluate to a resultant value

fn five() -> i32 {
    // the version with the semicolon will make this a statement, and error out
    //5;
    // an expression, since it returns a value
    5
}

fn plus_one(x: i32) -> i32 {
    return x + 1
}