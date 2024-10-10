use std::collections::HashMap;

fn main(){
    let mut users = HashMap::new();
    
    users.insert( String::from("vishal"), 23);
    users.insert(String::from("laula"),68);


    let first_user_age = users.get("laula");

    match first_user_age {
        Some(age) => println!("age is {}",age),
        None => println!("user not found"),
    }
}