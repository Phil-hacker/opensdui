use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;
use rocket::fs::relative;
use serde::Serialize;


#[derive(Serialize)]
struct Theme<'a> {
    background_color_1: &'a str,
    background_color_2: &'a str,
    foreground_color: &'a str,
    accent_color: &'a str,
}

impl Default for Theme<'_> {
    fn default() -> Self {
        Self {
            background_color_1: "#333",
            background_color_2: "#222",
            foreground_color: "#eee",
            accent_color: "#3b3",
        }
    }
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/style.css")]
fn style() -> Template {
    Template::render("style", Theme::default())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/css", rocket::fs::FileServer::from(relative!("/www/css")))
        .mount("/js", rocket::fs::FileServer::from(relative!("/www/js")))
        .mount(
            "/public",
            rocket::fs::FileServer::from(relative!("/www/public")),
        )
        .mount("/", routes![index, style])
}
