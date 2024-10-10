fn main(){
    let mut  s1 = String :: from("hello");
   
    println!("string from is {}  ", s1 );
    
    do_something(&mut s1);
}

fn do_something(s2:&mut String) {
    s2.push_str ( "My name is blabla ");
    println!("oh hello {} ", s2);
}







// if you want to pass refernce pass at&
// if you want to pass mutable refernce then add &mut
//borrowing example
// fn main(){
//     let s1 :String = String :: from("hello");
//     let s2 : &String = &s1 ;
//     println!("string from is {}  ", s1 );
//     do_something(s2:&s1);
// }