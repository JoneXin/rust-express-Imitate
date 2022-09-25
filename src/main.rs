use web_server::router::Router;
use web_server::xserver::app::App;
fn main() {
    let mut app = App::new();
    app.router("/user", Router::user_router);
    app.listen("0.0.0.0:4444").unwrap();
}
