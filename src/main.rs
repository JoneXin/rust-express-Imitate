use std::collections::HashMap;

use web_server::router::Router;
use web_server::xserver::app::App;
fn main() {
    test();
    let mut app = App::new();

    app.router("/user", Router::user_router);

    app.listen("0.0.0.0:4444").unwrap();
    println!("{}", 22222);
}

fn test() {

    // println!("{:#?}", header_map);
}
