mod about;
mod auth;
mod blog;
mod home;
mod navbar;
mod not_found;

use crate::about::About;
use crate::auth::{ForgotPassword, Login, Register, ResetPassword};
use crate::blog::Blog;
use crate::home::Home;
use crate::not_found::NotFound;
use dioxus::prelude::*;

static SERVICE_NAME: GlobalSignal<String> = Signal::global(|| (*shared::SERVICE_NAME).clone());
static SERVICE_DOMAIN: GlobalSignal<String> = Signal::global(|| (*shared::SERVICE_DOMAIN).clone());

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(crate::navbar::NavigationBar)]
        #[route("/")]
        Home {},
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
        #[route("/forgot-password")]
        ForgotPassword {},
        #[route("/reset-password?:code")]
        ResetPassword { code: String },
        #[route("/blog")]
        Blog {},
        #[route("/about")]
        About {},

        // wildcard route
        #[route("/:..endpoint")]
        NotFound { endpoint: Vec<String> },
}
