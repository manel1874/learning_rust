struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods
impl Rectangle {
    fn area_method(&self) -> u32 {
        self.width * self.height
    }
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");


    // Creating Instances From Other Instances With Struct Update Syntax

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // area of the rectangle
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "react1 is {:?}", rect1
    );


    println!(
        "react1 is {:#?}", rect1
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area_method()
    )

}
