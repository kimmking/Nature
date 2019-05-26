extern crate nature;

use dotenv::dotenv;

use nature::rpc::actix::{web_app};
use actix::System;
use actix_web::server;
use nature::actor::create_actors;

fn main() {
    dotenv().ok();
    let sys = System::new("http-server");

    create_actors();

    let server = server::new(|| web_app())
        .bind("127.0.0.1:8088")
        .unwrap();
    server.start();

    sys.run();
}

//fn main() {
//    sys_init();
//    let _ = rocket_server().launch();
//}


