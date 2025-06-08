use std::io;
use std::io::Read;

pub fn count_string(strs :&str)-> usize{
    strs.chars().count()
}
pub fn get_string(){
    let mut strs = String::new();
    println!("Please enter a string:");
    io::stdin().read_line(&mut strs).expect("Got nothing").to_string();
    
    println!("Given string is {}", strs);
    let counts = count_string(strs.as_str().trim_end());
    print!("The count of the strings is {}", counts);
    
}

