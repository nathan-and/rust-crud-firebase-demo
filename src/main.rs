use std::collections::HashMap;
use actix_web::{App, HttpServer, web};
use firebase_rs::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

struct AppState {
    firebase: Firebase,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Hash, PartialEq, Eq)]
struct Cat{
    name: String,
    color: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(||
        App::new()
        .app_data(web::Data::new(AppState {
            firebase: Firebase::new("https://d-physics-a50d4-default-rtdb.europe-west1.firebasedatabase.app/").unwrap() 
        }))
        .route("/cats", web::get().to(read_cats))
        .route("/cat", web::get().to(read_cat))
        .route("/cat", web::post().to(create_cat))
        .route("/cat", web::patch().to(update_cat))
        .route("/cat", web::delete().to(delete_cat))
        
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await

}

async fn create_cat(post: web::Json<Cat>, app: web::Data<AppState>) -> String{
    let firebase = app.firebase.at("cats");
    let _cats = firebase.set::<Cat>(&post).await;
    format!("{:?}", _cats.unwrap().data)
}

async fn read_cats(app: web::Data<AppState>) -> String {
    let firebase = app.firebase.at("cats");
    let cats = firebase.get::<HashMap<String, Cat>>().await.unwrap_or_default();
    println!("{:?}", cats);
    format!("{:?}", cats)
}


async fn read_cat(app: web::Data<AppState>) -> String{
    let firebase = app.firebase.at("cats").at(&app.id);
    let cat = firebase.get::<Cat>().await;
    format!("{:?}", cat)
}

async fn update_cat(app: web::Data<AppState>) -> String{
    let firebase = app.firebase.at("cats").at(&app.id);
    let _cat = firebase.update::<Cat>(&app.cat).await;
    format!("{:?}", _cat)
}

async fn delete_cat(app: web::Data<AppState>) -> String{
    let firebase = app.firebase.at("cats").at(&app.id);
    let _result = firebase.delete().await;
    format!("{:?}", _result)
}

// // convert a string to a response
// fn string_to_response(s: &str) -> Response{
//     serde_json::from_str(s).unwrap()
// }

// //convert a string to a user
// fn string_to_cat(s: &str) -> Cat{
//     serde_json::from_str(s).unwrap()
// }
