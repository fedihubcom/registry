use crate::states;
use crate::views;

use crate::i18n::I18n;
use crate::responses::CommonResponse;

use rocket::State;
use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/")]
pub fn index(
    i18n: State<I18n>,
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    let l10n = i18n.l10n("en").unwrap();

    let page_context = views::home::Index {
        i18n_fedihub: l10n.translate("fedihub").unwrap(),
        i18n_federated_services_without_censorship: l10n.translate("federated-services-without-censorship").unwrap(),
    };

    let context = views::Site {
        page: "home/index".to_string(),
        page_context,
        authenticity_token: csrf_token.authenticity_token().to_string(),
        current_user: current_user.0,
    };

    Ok(Template::render("site", &context))
}
