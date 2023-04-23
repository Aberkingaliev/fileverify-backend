use rocket::http::{Cookie, SameSite};

pub struct AuthUtils {}

impl AuthUtils {
    pub fn set_token_cookie(token_name: String, insertable_data: String) -> Cookie<'static> {
        let age = rocket::time::Duration::days(30);
        let cookies = Cookie::build(token_name, insertable_data)
            .same_site(SameSite::Strict)
            .path("/")
            .http_only(true)
            .max_age(age)
            .finish();

        return cookies;
    }
}
