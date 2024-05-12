use std::env;

fn main() {
    if let Err(_) = dotenvy::dotenv() {
        println!("Error loading .env file");
        return;
    }
    let Ok(msg) = env::var("MSG") else {
        println!("MSG not found in .env file");
        return;
    };
    println!("Message: {}", msg);
}
