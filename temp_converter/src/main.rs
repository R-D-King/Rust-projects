use std::io;

fn main() {
    println!("This is an app to Convert temperatures between Fahrenheit and Celsius.");
    'input_check: loop {
        println!("First select your temeprature scale: \n [1] from Fahrenheit to Celsius \n [2] from Celsius to Fahrenheit");

        let mut temp_type = String::new();

        io::stdin()  // takes input from the user as string
            .read_line(&mut temp_type)
            .expect("Failed to read line");
//  converting the string input to positive number 
        let temp_type: u8 = match temp_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if temp_type == 1 {
            println!("You Chose to convert from Fahrenheit to Celsius.\n");
            println!("Now enter your temprature in Fahrenheit:");
            let mut deg_num = String::new();
            io::stdin()
                .read_line(&mut deg_num)
                .expect("Failed to read line");

            let deg_num: f64 = match deg_num.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("{}", temp_converter(1, deg_num));
        } else if temp_type == 2 {
            println!("You Chose to convert from Celsius to Fahrenheit.\n");
            println!("Now enter your temprature in Celsius:");
            let mut deg_num = String::new();
            io::stdin()
                .read_line(&mut deg_num)
                .expect("Failed to read line");

            let deg_num: f64 = match deg_num.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("{}", temp_converter(2, deg_num));
        } else {
            println!("\nNo vaild option selected\n");  // when there is no valid selected option
            continue 'input_check;
        }

        break;
    }
}
// the conversion function
fn temp_converter(op: i8, deg: f64) -> f64 {
    if op == 1 {
        (deg - 32.0) / 1.8
    } else {
        (deg * 1.8) + 32.0
    }
}
