struct Rectangle{
    height: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self)-> i32{
        self.width * self.height 
    }
    // underscore is used as its value will not be used ut still its ther
    fn perimeter(&self , _: i32) -> i32 {
        2 * (self.width + self.height)
    }
    fn debug() -> i32 {
        return 1;
    }
}

fn main(){
   let rect1 = Rectangle {
    height :10 ,
    width: 20, 
   };
   println!("area is {}", rect1.area());
   println!("perimeter is {}", rect1.perimeter(1));
   println!("debug is {}", Rectangle::debug());
}