fn main(){
    let name = String::from("vishal");
    let length = get_str_length(name);
    println!("length is {}",length)
}

fn get_str_length(str :String) -> usize {
    return str.chars().count();
}