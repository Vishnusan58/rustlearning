struct User{
    username:String,
    email:String,
    password:String,
    active:bool
}

pub fn useds(){
    let users1 = User{
        username :String::from("Bishnu"),
        email:String::from("vishnusank"),
        password : String::from("nndsdin234"),
        active:true
    };
    println!("{:?}", users1.active)
}