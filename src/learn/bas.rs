use std::io;
pub fn bas() {
    println!("this is program to find is even or not ");
    println!("Enter a number to check if it is even or not");
    let mut nums  =String::new();
    io::stdin().read_line(&mut nums).expect("Failed to read line");
    println!("The numbers is {} and it is {}", nums, is_even(nums.trim().parse().unwrap()))
    }
pub fn is_even(num :i32) ->bool{
    if num %2 ==0{
        return true;
        }
    else {
        return false;
    }
    }

