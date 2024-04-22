use std::io;
use std::io::Write;
use std::process;

fn int(input: String)-> i32{
    // Parse input into a number
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            process::exit(0);
        }
    };
    number
}


fn main(){
    println!("\t\tCALCULATOR\n\n\tAVAILABLE ACTIONS\n1.Addition\n2.Substraction\n3.Multiplication\n4.Division");

    print!("\n\t:: ");

    let mut input = String::new();
    let mut first = String::new();
    let mut second = String::new();
    let result: i32;

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get the input!");
    input = input.trim().to_string(); 

    print!("Number 1: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut first)
        .expect("Failed to get the first number!");
    first = first.trim().to_string(); 
    
    print!("Number 2: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut second)
        .expect("Failed to get the second!");
    second = second.trim().to_string(); 


    match input.as_str(){
        "1" | "Addition" | "+"  => {
            result = int(first)+int(second);
        },
        "2" | "Substraction" | "-" => {
            result = int(first)-int(second);
        },
        "3" | "Multiplication" | "*" => {
            result = int(first)*int(second);
        },
        "4" | "Division" | "/" => {
            result = int(first)/int(second);
        },

        _ => {
            println!("You choose wrong choice!");
            process::exit(0);
        }
    }

    println!("\n\n\t Result: {}",result);
}
