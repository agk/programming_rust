fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("primer@example.com"),
        username: String::from("primerusername123"),
        active: true,
        sign_in_count: 1,
    }

    user1.email = String::from("antheremail@example.com");
    
}
