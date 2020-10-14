#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    parent: &'static str,
    users: Vec<&'static str>,
}

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
}

#[get("/")]
fn index() -> Template {
    let template_context = TemplateContext {
        parent: "layout",
        users: vec!["foo", "bar", "car"],
    };

    Template::render("index", &template_context)
}
