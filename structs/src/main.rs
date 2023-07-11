struct User {
    active: bool,
    name: String,
    email: String,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, r: Rectangle) -> bool {
        r.width < self.width && r.height < self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}
fn main() {
    let mut user = User {
        active: true,
        name: String::from("hello"),
        email: String::from("user@example.com"),
    };

    user.email = String::from("nice");

    println!("{} {}", user.name, user.email);

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    let r2: Rectangle = Rectangle {
        width: 5,
        height: 25,
    };
    println!("{:#?}", rect);
    println!("{}", rect.area());
    println!("Can hold: {}", rect.can_hold(r2));
}

fn build_user(email: String, name: String) -> User {
    User {
        active: true,
        name,
        email,
    }
}
