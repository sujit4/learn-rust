// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let guess: u32 = "42".parse().expect("string");

    println!("The value of x is: {guess}");
}