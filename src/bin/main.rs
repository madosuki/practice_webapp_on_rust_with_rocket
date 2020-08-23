#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate webapp;

use self::models::*;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use diesel::prelude::*;

use rocket::request::{Form, Request};
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

use webapp::*;

use self::schema::students::dsl::*;

#[derive(Serialize)]
struct AboutTemplate {
    value: String
}

#[post("/student", data = "<student>")]
fn modify_student(student: Form<Student>) -> Redirect {
    println!("{}", student.id);

    Redirect::to("/")
}

#[get("/regist")]
fn regist_student() -> Template {

    let mut tmp = HashMap::new();
    tmp.insert("", "");
    
    Template::render("form", &tmp)
}

#[get("/student?<_id>", format = "json")]
fn get_student_data(_id: usize) -> Option<Json<Student>> {

    let connection = connect_db();

    let tmp = _id as i32;

    let results: std::vec::Vec<Student> = students.filter(id.eq(tmp)).load::<Student>(&connection).expect("got error");

    for post in &results {
        println!("result: {}", post.name);
    }

    if results.len() > 0 {
        let data = results.get(0).unwrap();
        let student = Student {id: data.id, name: data.name.clone(), year: data.year, class_name: data.class_name.clone(), age: data.age, is_exist: data.is_exist};

        return Some(Json(student))
    }

    // if _id > 0 {
    //     let student = Student {id: 1, name: "Rei".to_string(), year: 2, class_name: "B".to_string(), age: 17, is_exist: true };

    //     return Some(Json(student))
    // }

    None
 
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/about")]
fn about() -> Template {
    let value = "about";

    let template = AboutTemplate { value: value.to_string() };

    Template::render("about", &template)
}

#[get("/user/<_id>")]
fn user(_id: usize) -> String {
    let id_str = _id.to_string();
    let mut tmp = "<h1>your id: ".to_owned();
    let end = "</h1>";

    tmp.push_str(&id_str);
    tmp.push_str(&end);
    
    tmp
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite().mount("/", routes![index, about, user, get_student_data, modify_student, regist_student]).attach(Template::fairing()).register(catchers![not_found]).launch();
}
