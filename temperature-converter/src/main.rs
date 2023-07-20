use std::io;

fn main() {
    println!("Temperature Converter");
    println!("Please choose the unit type, C or F: ");
    let mut unit_type = String::new();
    io::stdin()
        .read_line(&mut unit_type)
        .expect("Failed to read line");
    
    println!("Choose a temperature");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        },
    };
    let converted_temp = match unit_type.trim() {
        "C" => (temperature * 9.0/5.0) + 32.0,
        "F" => (temperature - 32.0) * 5.0/9.0,
        _ => {
            println!("Please type C or F!");
            return;
        },
    };
    println!("Converted temperature: {converted_temp}");
}
