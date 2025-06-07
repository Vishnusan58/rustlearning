pub fn vector_examples() {
    // Creating a new, empty vector
    let v: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v);
    
    // Creating a vector with initial values using the vec! macro
    let v = vec![1, 2, 3, 4, 5];
    println!("Vector with initial values: {:?}", v);
    
    // Adding elements to a vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector after pushing elements: {:?}", v);
    
    // Accessing elements
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    // Safe access with get method
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // Iterating over a vector
    println!("Iterating over vector:");
    for i in &v {
        println!("{}", i);
    }
    
    // Modifying elements in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Vector after modification: {:?}", v);
}