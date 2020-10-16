use crate::states;

use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/sign_in")]
pub fn new(
    current_user: states::CurrentUser,
) -> Result<Template, Redirect> {
    if let Some(_) = current_user.0 {
        return Err(Redirect::to(uri!(super::home::index)));
    }

    Ok(Template::render("sessions/new", &BasicTemplateContext {
        layout: "site",
    }))
}

#[derive(Serialize)]
struct BasicTemplateContext {
    layout: &'static str,
}
