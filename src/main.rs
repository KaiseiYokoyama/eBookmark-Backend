#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;

mod routes;
mod prelude;
mod structs;

fn main() {
    launch();
}

fn launch() {
    use rocket::config::{Config, Environment};

    let config = Config::build(Environment::Development)
        .address("localhost")
        .port(8080)
        .finalize().unwrap();

    rocket::custom(config)
        .mount("/register", routes![routes::register::register])
        .mount("/delete", routes![routes::delete::delete])
        .mount("/search", routes![routes::search::search])
        .mount("/get", routes![routes::get::one::get_one, routes::get::all::get_all])
        .attach(rocket_contrib::templates::Template::fairing())
        .launch();
}