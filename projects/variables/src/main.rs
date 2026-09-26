fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = x + 1;
    println!("The value of x is: {x}");
    
    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let y = 5;
    println!("The value of y is: {y}");
    let y = y + 1;
    println!("The value of y is: {y}");

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // shadowing is useful here to reuse the same spaces variable
    // instead of spaces_str and spaces_num
    let spaces = "   ";
    println!("Spaces: {spaces}");
    let spaces = spaces.len();
    println!("Spaces: {spaces}");
}
