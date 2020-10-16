use crate::database;
use crate::models;

use rocket::request::{FromRequest, Outcome, Request};

pub struct MaybeCurrentUser(pub Option<models::User>);

impl<'current_user, 'request>
    FromRequest<'current_user, 'request>
    for MaybeCurrentUser
{
    type Error = ();

    fn from_request(request: &'current_user Request<'request>)
        -> Outcome<Self, Self::Error>
    {
        let db_conn = request.guard::<database::DbConn>()?;

        let user = request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok())
            .and_then(|user_id| models::User::find(db_conn, user_id).ok());

        Outcome::Success(MaybeCurrentUser(user))
    }
}
