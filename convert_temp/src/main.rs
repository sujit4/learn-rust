// convert temperatures between fahrenheit and celsius
use text_io::scan;

fn main() {
    let f_mult_factor: f32 = 9.0/5.0;
    let c_mult_factor: f32 = 5.0/9.0;
    let common_factor: f32 = 32.0;

    println!("Enter convertion option below: \n * Celsius to Fahrenheit - enter 1 \n * Fahrenheit to Celsius - enter 2");
    let choice: i32;
    scan!("{}", choice);    

    if choice == 1 {
        println!("Enter the temperature in Celsius:");
        let cel_temp: f32;
        scan!("{}",cel_temp);
        
        let res = (cel_temp * f_mult_factor) + common_factor;
        println!("The temperature in Fahrenheit is {}", res.round())

    } else if choice == 2 {
        println!("Enter the temperature in Fahrenheit:");
        let fah_temp: i32;
        scan!("{}", fah_temp);

        let res = (fah_temp as f32 - common_factor) * c_mult_factor;
        println!("the temperature in Celsius is {}", res);

    } else {
        println!("invalid input");
    }
}