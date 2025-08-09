fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("primer@example.com"),
        username: String::from("primerusername123"),
        active: true,
        sign_in_count: 1,
    }
    
}
