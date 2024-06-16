#[macro_use]
extern crate rocket;

mod routes;
mod schemas;
mod services;

use routes::date::date_plus_month;
use routes::date::get_current_date;

#[get("/check")]
fn health_check() -> &'static str {
    "It's live!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![health_check, date_plus_month, get_current_date],
    )
}
