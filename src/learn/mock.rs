pub fn looping(){
    use std::time::Instant;
    let start = Instant::now();
    let mut x : u16 = 10;
     for i in 10 .. 1000{
         let x = 10 + i;
         println!("{}",x);
     }
    let duration = start.elapsed();
    print!("Time taken for the loop:{}", duration.as_secs())
}