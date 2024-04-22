use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use std::io::Write;


fn get_last_three_digits(number: u64) -> u64 {
    let last_three_digits = number % 100;
    last_three_digits
}

fn main(){

    let current_time = SystemTime::now();

    // Get the duration since the Unix epoch
    let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Convert the duration to seconds
    let unix_time = duration_since_epoch.as_secs();

    let last_three_digits = get_last_three_digits(unix_time) as i32;

    println!("\t****Hint:**** ");

    match last_three_digits{
        1..=10 => println!("The number is between 1 and 10"),
        11..=20 => println!("The number is between 11 and 20"),
        21..=30 => println!("The number is between 21 and 30"),
        31..=40 => println!("The number is between 31 and 40"),
        41..=50 => println!("The number is between 41 and 50"),
        51..=60 => println!("The number is between 51 and 60"),
        61..=70 => println!("The number is between 61 and 70"),
        71..=80 => println!("The number is between  71 and 80"),
        81..=90 => println!("The number is between 81 and 90"),
        91..=99 => println!("The number is between 90 and 99"),

        _ => panic!("The program is misbehaving!"),
    }

    println!("    ---Only 3 Chances--- \n"); // Separate 'Hint' from the Game

    let mut input = String::new(); 
    let mut attempts: Vec<i32> = Vec::new();
    let mut number: i32;

    for i in 1..=3{
        // Display a prompt to the user
        print!("Guess Number: ");
        io::stdout().flush().unwrap(); 

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return; // Exit the program if input cannot be parsed
            }
        };

        if number==last_three_digits{
            println!("\n\tCONGRATS!, The correct number is: {}",number);
            attempts.push(number);
            break;
        }else{
            println!("Wrong Guess!, Try Again");
            attempts.push(number);
            input="".to_string();

            if i==3{
                println!("\n\tYou Lost the Game!, Correct number was: {}",last_three_digits);
                break;
            }
        }
    }


    print!("\n\tYour attempts were: ");
    for i in &attempts{
        print!("{} ",i);
    }

   
    println!("\n\t\tGame Finished!");


}
