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

fn main() {
    println!("Hello, world!");

    let mut user_1 = User {
        username: String::from("kapitoshqa"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true
    };
    println!("{:?}", user_1);

    let user_2 = User {
        /*
        Создание нового экземпляра структуры User с использованием 
        некоторых значений из user1 */
        email: String::from("another@example.com"), 
        username: String::from("anotherusername567"),
        active: user_1.active,
        sign_in_count: user_1.sign_in_count,
    };

    println!("{:?}", user_2);
    user_1.active = false;
    println!("{:?}", user_2);
    println!("{:?}", user_1);

    let user_3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user_1
    };
    println!("{:?}", user_3);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let width1 = 30;
    let height1 = 50;
    println!("Площадь прямоугольника равна {} квадратным пикселам.", area(width1, height1));

    let rect_1 = (30, 50);
    println!("Площадь прямоугольника равна {} квадратным пикселам.", area_v2(rect_1));  
    let rect_2 = Rectangle {
        width: 30,
        height: 50
    };
    println!("Площадь прямоугольника равна {} квадратным пикселам.", area_v3(&rect_2)); 
    let rect_3 = Rectangle {
        ..rect_2
    };
    println!("Площадь прямоугольника равна {} квадратным пикселам.", rect_3.area());
    println!("Может ли rect1 содержать в себе rect3? {}", rect_3.can_hold(&rect_3));
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_v2(email: String, username: String) -> User {
    /*Данная запись отличается от предыдущей тем, что мы не дублируем email: email.
    При одинаковом названии аргументов, можно явно их не указывать */
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v2(dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}