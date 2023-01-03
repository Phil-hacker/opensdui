use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing()).mount("/", routes![index])
}