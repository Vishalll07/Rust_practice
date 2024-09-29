//returns an none or null or nill value ,
// fn that can return 1 or null this where options comes you 
//can give option in function parameter

fn main(){
    let index = find_first(String ::from("vishal"));

    match index {
        Some(value) => println!("index is {}", value),
        None => println!("a not found"),
    };
}

fn find_first(s: String) -> Option<usize> {
    for(index , c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(index);
        }
    }
    return None;
}