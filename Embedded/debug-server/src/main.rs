#[macro_use] extern crate rocket;

// Post request that revieves and prints logs
#[post("/log", data = "<log_data>")]
fn log(log_data: String) -> String {
    println!("Received log data: {}", log_data);
    "Log received".to_string()
}
// lisen on 0.0.0.0:8000
#[launch]
fn rocket() -> _ {
    let address = "0.0.0.0";
    println!("Listening on {}", address);
    rocket::build().configure(rocket::Config::figment()
        .merge(("address", address)))
        .mount("/", routes![log])
}
