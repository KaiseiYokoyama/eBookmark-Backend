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

use prelude::*;

fn main() {
    println!("Hello, world!");
}
