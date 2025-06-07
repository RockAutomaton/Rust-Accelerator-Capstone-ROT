use std::env;


fn main() {
    // Read the environment variable or use a default
    let _ = dotenvy::dotenv();

    let api_url = env::var("ROT_API_URL").unwrap().to_string();

    // Pass to the compiler
    println!("cargo:rustc-env=ROT_API_URL={}", api_url);
} 