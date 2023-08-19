#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    }
    
}



fn main() {

    let user = User {
        username: String::from("anand"),
        email: String::from("anand@123"),
        sign_in_count: 1,
        active: true
    };

    print!("{:#?}", user);
    let rect = Rectangle{
        width: 30,
        height: 50
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40
    };
    println!("Area of rectangle is {}", rect.area());
    println!("Can rect hold?: {}", rect.can_hold(&rect2));
    let sq = Rectangle::square(10);
    println!("Square is {:#?}", user.email);
}

