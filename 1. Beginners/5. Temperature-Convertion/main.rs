use std::io;
use std::io::Write;

fn int(number: String)->i32{
    match number.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            return 0;
        }
    }
}

fn main(){
    let mut input = String::new();
    let mut _num:i32;
    let  choice:String;

    println!("\n\t\t CELSIUS/FAHRENHEIT CONVERTION PROGRAM!\n\n\t\t1. Celsius -> Fahrenheit.\n\t\t2. Fehrenheit -> Celsius.");

    print!("\t\t  (Choice)-> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get choice input!");
    input = input.trim().to_string();

    choice = input.clone();

    input.clear();

    print!("\t\t  (Number)-> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get Number input!");
    input = input.trim().to_string();


    match choice.as_str(){
        "1" =>{
            println!("Answer: {}F",(9/5)*int(input)+32);
        },
        "2" =>{
            println!("Answer: {}C",(int(input)-32)*(5/9));
        },
        _ =>{
            println!("Wrong Input!!");
        }
    }
}
