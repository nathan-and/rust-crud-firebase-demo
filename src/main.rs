mod api;

use actix_web::{App, HttpServer, web};
use firebase_rs::*;
use api::cat::{
    read_cat,
    read_cats,
    create_cat,
    update_cat,
    delete_cat
};

struct AppState {
    firebase: Firebase,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(||
        App::new()
        .app_data(web::Data::new(AppState {
            firebase: Firebase::new("YOUR FIREBASE END POINT HERE").unwrap() 
        }))
        .service(read_cat)
        .service(read_cats)
        .service(create_cat)
        .service(update_cat)
        .service(delete_cat)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await

}