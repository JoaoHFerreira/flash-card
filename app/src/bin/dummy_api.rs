#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .configure(rocket::Config {
            port: 8000,
            address: [0, 0, 0, 0].into(),
            ..Default::default()
        })
}
