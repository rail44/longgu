#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use] extern crate serde_derive;
extern crate serde_json;


pub mod schema;
pub mod connection;
