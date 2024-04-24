/* 
*   THE TO-DO
*   APPLICATION
*
* 
*/
use std::fs;
use std::fs::File;

use std::io;
use std::io::{Write,Read};


// use std::io::Result;

fn lists_files_in_folder(folder_path: &str)-> Result<Vec<String>, io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(folder_path)?{
        let entry = entry?;
        let file_name = entry.file_name();
        files.push(file_name.to_string_lossy().to_string());
    }
    Ok(files)
}

fn write_notes(filename: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}


fn show_files(notes_path: &str){
    println!("\nYOUR NOTES");
    let mut _display = String::new();
    let mut _count:i32 =1;

    match lists_files_in_folder(notes_path){
        Ok(files)=>{
            for file in files{
                _display = file[..file.len() - 4].to_string();
                println!("\t{}. {}",_count, _display);
                _count = _count+1;
            }
        }
        Err(_)=>{
            println!("\t\tNo NoteBook was Found!");
        }
    }
}

fn remove_file(file_id: i32, notes_path: &str){
    println!("\nYOUR NOTES");
    let mut _count:i32 =1;

    match lists_files_in_folder(notes_path){
        Ok(files)=>{
            for file in files{
                if file_id==_count{
                   let _ = fs::remove_file(format!("storage/{}",file)); // ignore errors thrown
                }
                _count = _count+1;
            }
        }
        Err(_)=>{
            println!("\t\tNotebook was not removed!");
        }
    }
}

fn read_contents(file_path: &str)->Result<String, std::io::Error>{
    let mut file=File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)

}
fn read_file(file_id: i32,notes_path: &str){
    let mut _display = String::new();
    let mut _count:i32 =1;

    match lists_files_in_folder(notes_path){
        Ok(files)=>{
            for file in files{
                if file_id==_count{
                  _display = file[..file.len() - 4].to_string();
                   let whole_filename = format!("storage/{}",file);
                   
                   match read_contents(whole_filename.as_str()){
                    Ok(contents)=>println!("\n\t\t┌──({})\n\t\t└─| {}",file,contents),
                    Err(_)=>println!("Failed to read the file!"),
                   }
                }
                _count = _count+1;
            }
        }
        Err(_)=>{
            println!("Failed to read the file!");
        }
    }
}

fn main(){
    let notes_path = "storage";


    // ACTIONS
    let mut _action:i32 = 0;



    println!("\t\t YOUR NOTEBOOK SYSTEM\n");
    loop{
        let mut input = String::new();
        println!("\n\nChoose ACTIONS:\n\t1. View Notes\n\t2. See Notes\n\t3. Create A NoteBook\n\t4. Delete Notes");
        
        print!("    ->");
        // Take the action 
        io::stdout().flush().unwrap(); // Clear buffer
        io::stdin()
            .read_line(& mut input)
            .expect("Failed to readline!");
        input = input.trim().to_string(); // clear new line
        _action = match input.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("Invalid Input!");
                break;
            }
        };

        match _action {
            1 => {
                input.clear();
                show_files(notes_path);
                
            },
            2=>{
                input.clear();
                show_files(notes_path);
                print!("    -> ");

                // Take the action 
                io::stdout().flush().unwrap(); // Clear buffer
                io::stdin()
                    .read_line(& mut input)
                    .expect("Failed to readline!");
                input = input.trim().to_string(); // clear new line
                _action = match input.trim().parse() {
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid Input!");
                        break;
                    }
                };
                read_file(_action,notes_path);
                  
            },
            3 =>{
                input.clear();

                print!("    NoteBook Title-> ");

                // Take the action 
                io::stdout().flush().unwrap(); // Clear buffer
                io::stdin()
                    .read_line(& mut input)
                    .expect("Failed to readline!");
                input = input.trim().to_string(); // clear new line
                let mut _title=input.clone();

                input.clear();

                print!("    Notes-> ");

                // Take the action 
                io::stdout().flush().unwrap(); // Clear buffer
                io::stdin()
                    .read_line(& mut input)
                    .expect("Failed to readline!");
                input = input.trim().to_string(); // clear new line
                
                let mut _contents=input.clone();

                let extension = ".txt";
                let filename = format!("storage/{}{}",_title,extension);

                match write_notes(filename.as_str(),_contents.as_str()){
                    Ok(())=> println!("\t Done!"),
                    Err(_)=> println!("Error happened!"),
                };
                

            },
            4=>{
                input.clear();
                println!("\t\t DELETING YOUR NOTES");
                show_files(notes_path);
                print!("    -> ");

                // Take the action 
                io::stdout().flush().unwrap(); // Clear buffer
                io::stdin()
                    .read_line(& mut input)
                    .expect("Failed to readline!");
                input = input.trim().to_string(); // clear new line
                _action = match input.trim().parse() {
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid Input!");
                        break;
                    }
                };

                remove_file(_action,notes_path);
                println!("\t Done!\n");

                
                
            },
            _ => {
                println!("Wrong Choice!");
                break;
            }
        }
        
    }
    



    
}
