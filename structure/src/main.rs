struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    let user1 = User {
        email: "primer@example.com",
        username: "primerusername123",
        active: true,
        sign_in_count: 1,
    }

}
