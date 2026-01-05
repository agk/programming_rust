struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    let mut user1 = User {
        email: String::from("primer@example.com"),
        username: String::from("primerusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut _user2 = build_user(String::from("anotheremail@example.com"), String::from("agk"));

    // Использование кортежных структур без именованных полей для создания разных типов
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
