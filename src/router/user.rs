use super::Xrouter;
use crate::server::user::User;

pub struct UserRouer {}

impl UserRouer {
    pub fn router(route: &str) {
        Xrouter::get("/add", User::add_user);
        Xrouter::get("/add", User::add_user);
    }
}
