struct User {
    firstname: String ,
    lastname: String,
    age: i32,
}

fn main(){
    let user = User {
        firstname: String::from("heisenburgh"),
        lastname: String::from("say my name"),
        age: 32,
    };
    println!("{}",user.lastname);
    println!("{}",user.firstname);
    println!("{}",user.age);
}