use chrono::{Local, Utc};

fn main(){
    //get the current time and date in UTC
    let now = Utc::now();
    println!("current date and time in UTC: {}",now);

    //format date and Time
    let fromatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("formated date and time: {}", fromatted);

    //get local time
    let local = Local::now();
    println!("current date and Time in Local: {}", local);

}