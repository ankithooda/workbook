struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    let user1 = build_user(String::from("ankit"), String::from("a@a.com"), true, 34);
    println!("{} {} {}", user1.name, user1.email, user1.sign_in_count);

    let user2 = build_user(user1.name, user1.email, user1.active, user1.sign_in_count);
    println!("{} {} {}", user2.name, user2.email, user2.sign_in_count);

    println!("{} {}", user1.active, user1.sign_in_count);

}

fn build_user(name: String, email: String, active: bool, sign_in_count: u64) -> User {
    User {
        active,
        name,
        email,
        sign_in_count
    }
}