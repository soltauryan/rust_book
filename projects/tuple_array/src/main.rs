use std::io;


fn main() {
    let tup = (500, 6.4, 1);

    let (a, y, z) = tup;

    println!("the value of y is {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred}, {six_point_four}, {one}");

    // Arrays, fixed length, all same type
    // useful when you know the number of elements, and they won't change, like months
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[2]);
    let b = [3; 5];
    println!("b{:#?}", b);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    invalid_array()

}



fn invalid_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
