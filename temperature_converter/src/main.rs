use std:: io;

fn main() {
    println!("Temperature converter");
    println!("Enter temperature:");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let unit:&str = input        
        .trim_matches(char::is_whitespace)
        .trim_left_matches(char::is_numeric)
        .trim(); 

        
    println!("Unit is {}",unit); 
    

    let temperature: i32 = input
        .trim_matches(char::is_whitespace)
        .trim_end_matches(char::is_alphabetic) 
        .trim()
        .parse()
        .expect("Please type a number!");
    
    let mut answer = 0;
    let mut unit_answer = String::new();
    match unit{
        "C" => {
            answer = convert_to_fahrenheit(temperature);
            unit_answer = "F".to_string();
        },
        "F" => {
            answer = convert_to_celsius(temperature);
            unit_answer = "C".to_string();
            },
        _ =>{},
    }    
    
    println!("This is {} {}",answer, unit_answer);   
}

fn convert_to_fahrenheit(celsius: i32) -> i32
{
    celsius * 9 / 5 +32
}

fn convert_to_celsius(fahrenheit: i32) -> i32
{
    (fahrenheit - 32) * 5 / 9
}
