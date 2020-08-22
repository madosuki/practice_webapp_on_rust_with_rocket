use serde::{Serialize, Deserialize};
use diesel::Queryable;
use rocket::request::FromForm;

#[derive(Eq, PartialEq, serde::Serialize, serde::Deserialize, Debug, rocket::request::FromForm, Queryable)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub year: i32,
    pub class_name: String,
    pub age: i32,
    pub is_exist: bool
}
