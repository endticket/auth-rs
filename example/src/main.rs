#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_simpleauth as auth;
extern crate rocket;

use auth::userpass::UserPass;
use rocket::request::Form;
use rocket::http::Cookies;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::DummyAuthenticator;

#[get("/admin")]
fn admin(user: UserPass<DummyAuthenticator>) -> String{
	// we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
	format!("Restricted administration area, user logged in with ID: {}", user.user_id)
}

#[get("/admin")]
fn login() -> &'static str{
	"Login page"
}

#[post("/admin", data = "<form>")]
fn login_post(form: Form<LoginStatus<DummyAuthenticator>>, cookies: Cookies) -> LoginRedirect{
	// creates a response with either a cookie set (in case of a succesfull login)
	// or not (in case of a failure). In both cases a "Location" header is send.
	// the first parameter indicates the redirect URL when successful login,
	// the second a URL for a failed login
	form.into_inner().redirect("/admin", "/admin", cookies)
}

fn main(){
    // main setup code
}
