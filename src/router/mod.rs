use crate::server::user::User;
use crate::xserver::router::XRouter;
pub struct Router {}

impl Router {
    pub fn user_router(x_router: &mut XRouter) {
        x_router.get("/query", User::get_user);
        x_router.post("/add", User::add_user);
    }
}
