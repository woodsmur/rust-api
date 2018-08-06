#![feature(plugin, custom_derive, const_fn, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


mod models;
mod schema;
mod db;
mod static_files;


fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![static_files::all, static_files::index])
}

fn main() {

//    let book = models::NewBook {
//        title: String::from("Gravity's Rainbow"),
//        author: String::from("Thmoas Pynchon"),
//        published: true,
//    };
//
//    if models::Book::insert(book, &conn) {
//        println!("sucess");
//    } else {
//        println!("failed");
//    }

    rocket().launch();

}
