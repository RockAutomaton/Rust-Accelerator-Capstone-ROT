use std::env;

fn main() {
    // Read the environment variable or use a default
    let _ = dotenvy::dotenv();

    // Print all environment variables for debugging
    println!("cargo:warning=Environment variables:");
    for (key, value) in env::vars() {
        println!("cargo:warning={}={}", key, value);
    }

    // Try to get the API URL, with a default if not set
    let api_url = match env::var("ROT_API_URL") {
        Ok(url) => {
            println!("cargo:warning=Found ROT_API_URL: {}", url);
            url
        },
        Err(e) => {
            println!("cargo:warning=ROT_API_URL not found: {}. Using default.", e);
            "http://localhost:8080".to_string()
        }
    };

    // Pass to the compiler
    println!("cargo:rustc-env=ROT_API_URL={}", api_url);
} 