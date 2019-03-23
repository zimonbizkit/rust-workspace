use std::io;

const CELSIUS :char = 'C';
const FARENHEIT :char = 'F';

// temperature converter just for testing expressions in functions
fn main() {
    let mut input = String::new();
    let mut result:i32 = 0;
    let mut dest:char ='T';

    println!("Celsius to Farenheit converter!");

    loop {
        println!("Enter a valid integer followed by a valid temperature measure unit (C for Celsius, F for Farenheit)");
        io::stdin().read_line(&mut input);
        
        if input.trim() =="quit" {
            break;
        }

        match input.trim().chars().rev().next().unwrap() {
            CELSIUS => {
                result = convert_to_farenheit(input[..input.len()-2].parse::<i32>().unwrap());
                dest = 'F';
            },
            FARENHEIT =>{
                result = convert_to_celsius(input[..input.len()-2].parse::<i32>().unwrap());
                dest = 'C';
            },
            _ => {
                println!("Temperature measure unit '{}' is wrong",input.trim().chars().rev().next().unwrap());
                continue;
            },
        }

        println!("Conversion result is {}º{}",result,dest);
        input.clear();
    }
}

fn convert_to_farenheit(temperature:i32) -> i32 {
    (temperature * 9/5) + 32
}

fn convert_to_celsius(temperature:i32) -> i32 {
    (temperature - 32) * 5/9
}
