use rand::Rng;

fn generate_password(length: usize) -> String {
    let password: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    password
}

fn main() {
    let password = generate_password(10);
    println!("Your new password is: {}", password);
}
