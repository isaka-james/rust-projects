// This are beginners examples from the book 'Comprehesive Rust' from Google Developers

fn rust_default_types(){
    let _x=3.14; // Rust will default to f64
    let _y=7; // Rust will default to i32

    // Uncomment, Will throw error trying to compare float and integer
    //  Though you need to change the above _x,_y to x and y respectively before uncommenting
    // assert_eq!(x,y); // You are trying to compare the {float} and {integer}
    println!("An error has been simulated, because of the equivalenting different variable types!");
}

fn finabocci_series(var_x: u32)->u32{
    if var_x <= 2 {
        return var_x;
    }else{
        return var_x + finabocci_series(var_x-1);
    }
}

fn main(){
    // Lesson from chapter one
   rust_default_types();

   let number_for_finabocci:u32 = 20; 
   println!("The finabocci series for 20 is: {}",finabocci_series(number_for_finabocci));
}
