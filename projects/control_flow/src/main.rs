fn main() {
    let number = 1;

    if number > 5 {
        println!("condition was true");
    } else if  number == 1 {
        println!("HOLY SHIT");
    } else {
        println!("condition was false");

    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number)
}
