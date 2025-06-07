pub fn tuple_examples() {
    // Basic tuple with different types
    let tup: (i32, f64, &str) = (500, 6.4, "hello");
    println!("Basic tuple: {:?}", tup);

    // Accessing tuple elements using indexing
    println!("First element: {}", tup.0);
    println!("Second element: {}", tup.1);
    println!("Third element: {}", tup.2);

    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);

    // Nested tuples
    let nested_tuple = (42, (3.14, "nested"), true);
    println!("Nested tuple: {:?}", nested_tuple);
    println!("Accessing nested element: {}", (nested_tuple.1).0);

    // Empty tuple (unit)
    let empty_tuple = ();
    println!("Empty tuple (unit): {:?}", empty_tuple);

    // Tuple with a single element (needs a comma)
    let single_element = (42,);
    println!("Single element tuple: {:?}", single_element);
}
