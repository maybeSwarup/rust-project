use std::fmt::Debug;

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width * self.height)
    }

    // fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    //     write!(f, "The rectangle prints like this {}.", self.width * self.height)
    // }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    
    println!("The area of the rectangle is {}", rect.area());
    println!("The perimeter of the rectangle is {}", rect.perimeter());
    // println!("{:?}", rect);
}