use rocket::serde::json::Json;

use crate::{schemas::date::Date, services};

#[get("/date/current")]
pub fn get_current_date() -> Json<Date> {
    let response = services::date::get_current_date();
    Json(response)
}

#[post("/date/plus-month", data = "<date>")]
pub fn date_plus_month(date: Json<Date>) -> Json<Date> {
    let response = services::date::date_plus_month(date.into_inner());
    Json(response)
}
