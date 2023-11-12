fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("cevauser"),
        email: String::from("string@apostroph.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
