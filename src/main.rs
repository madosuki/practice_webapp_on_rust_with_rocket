#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use rocket::request::{Form, Request};
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize, Debug, FromForm)]
struct Student {
    id: u32,
    name: String,
    year: u8,
    class_name: String,
    age: u32
}

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

#[get("/student?<id>", format = "json")]
fn get_student_data(id: usize) -> Option<Json<Student>> {

    if id > 0 {
        let student = Student {id: 1, name: "Rei".to_string(), year: 2, class_name: "B".to_string(), age: 17 };

        return Some(Json(student))
    }

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

#[get("/user/<id>")]
fn user(id: usize) -> String {
    let id_str = id.to_string();
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
