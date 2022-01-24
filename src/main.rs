use numgle::converter::{self, data::DataSet, converts::{Converter, self}};
use rocket::{serde::json::serde_json, State};
use std::{fs, sync::Arc, convert};
#[macro_use] extern crate rocket;



#[get("/<input>")]
fn index(input:&str,state: &State<Converter>) -> String {
    state.convert_str(input)
}

#[launch]
fn rocket() -> _ {
    let dataset = fs::read_to_string("dataset.json").expect("dataset file not found");
    let d: DataSet = serde_json::from_str(&dataset).expect("parsing json fail");
    let c = converts::Converter::new(d);
    rocket::build().manage(c).mount("/", routes![index])
}