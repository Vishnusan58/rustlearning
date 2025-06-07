pub fn vectorsd(){
    println!("TuplesList ");
    
    let tups :(&str,i32,f64 , bool) = ("Helo",12, 3.14, true);
    println!("this values in the  tuples is {}, {}, {}, {}", 
             tups.0, tups.1, tups.2, tups.3);
    
}
