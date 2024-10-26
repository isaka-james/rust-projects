// This are beginners examples from the book 'Comprehesive Rust' from Google Developers

fn rust_default_types(){
    let _x=3.14; // Rust will default to f64
    let _y=7; // Rust will default to i32

    // Uncomment, Will throw error trying to compare float and integer
    //  Though you need to change the above _x,_y to x and y respectively before uncommenting
    // assert_eq!(x,y); // You are trying to compare the {float} and {integer}
    println!("An error has been simulated, because of the equivalenting different variable types!");
}

fn finabocci_series(n: u32)->u32{
    if n <= 2 {
        return n
    }

    return n + finabocci_series(n-1);
}

fn outer_and_inner(){
    // This checks the first student to have the 
    let student_marks = [ [60,62,34],[56,45,70],[100,67,23],[34,34,12]];
    'outer: for i in 0..3{
        println!("This is the {i} student marking!");
        'inner: for j in 0..3{
            if student_marks[i][j] == 100 {
                println!("Got a student who hit: marks={}, Stopping marking rightaway",student_marks[i][j]);
                break 'outer;
            }else if student_marks[i][j] >= 50 {
                println!("Student {i} is doing good: marks={}!",student_marks[i][j]);
            }else if student_marks[i][j] < 50 {
                println!("This student is doing very bad, Stopping accessing his other marks. his marks={}!",student_marks[i][j]);
                break 'inner;
            }
        }
        println!("Going for another student.");
    }
}

fn main(){
    // Lesson from chapter one
   rust_default_types();

   let number_for_finabocci:u32 = 20; 
   println!("The finabocci series for 20 is: {}",finabocci_series(number_for_finabocci));

   // Using outer and inner
   outer_and_inner();
}
