use std::io;

fn main() {
    println!("Convert the temperature!");

    println!("Select a temperature. fahrenheit (0), or celsius (1):");

    let mut user_temperature = String::new();
    
    io::stdin()
    .read_line(&mut user_temperature)
    .expect("Failed to read line");

    println!("You selected: {user_temperature}");

     let user_temperature: u32 = user_temperature.trim().parse().expect("Please type a number!");
    
    if user_temperature == 0 {
        println!("Please input your temperature in fahrenheit");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line!");
        

        let fahrenheit: u32 = user_input.trim().parse().expect("msg");

        println!("You are converting {fahrenheit}");

        let celsius = fahrenheit * 2;

        println!("the Celsius value is {celsius}");
        

        // println!("Do you want to convert more?");

        // let user_decision = String::new();
        // println!("{user_decision}");
    } else if user_temperature == 1 {
        println!("Please input your temperature in celsius");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read line!");
        

        let celsius: u32 = user_input.trim().parse().expect("msg");

        println!("You are converting {celsius}");

        let fahrenheit = celsius * 4;

        println!("the Celsius value is {fahrenheit}");
        

        // println!("Do you want to convert more?");

        // let user_decision = String::new();
        // println!("{user_decision}");
    }   
}

// fn celsius_to_fahrenheit(celsius: f64) -> f64 {
//     celsius * 1.1
// }

// fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
//     fahrenheit * 1.7
// }