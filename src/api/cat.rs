use std::collections::HashMap;
use derive_more::Display;
use actix_web::{
    get, 
    post, 
    put,
    delete,
    error::ResponseError,
    web::{Json, self},
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use crate::AppState;

#[derive(Serialize, Deserialize, Debug)]
struct Cat{
    name: String,
    color: String
}

#[derive(Debug, Display)]
pub enum CatError {
    CatNotFound,
    CatUpdateFailure,
    CatCreationFailure,
    BadTaskRequest
}

impl ResponseError for CatError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CatError::CatNotFound => StatusCode::NOT_FOUND,
            CatError::CatUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            CatError::CatCreationFailure => StatusCode::FAILED_DEPENDENCY,
            CatError::BadTaskRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/cat")]
async fn read_cats(app: web::Data<AppState>) -> Result<Json<HashMap<String, Cat>>, CatError>  {
    println!("Read Cats");
    let firebase = app.firebase.at("cats");
    let cats = firebase.get::<HashMap<String, Cat>>().await
    .and_then(|entries| { 
        Ok(entries)
    });
    println!("{:?}", cats);
    match cats {
        Ok(cats) => Ok(actix_web::web::Json(cats)),
        Err(_) => Err(CatError::BadTaskRequest)
    }
}

#[get("/cat/{id}")]
async fn read_cat(path: web::Path<String>, app: web::Data<AppState>) -> Result<Json<Cat>, CatError>  {
    let id = path.into_inner();
    println!("Read Cat {:?}", id);
    let firebase = app.firebase.at("cats").at(&id);
    let cat = firebase.get::<Cat>().await
    .and_then(|entries| { 
        Ok(entries)
    });
    println!("{:?}", cat);
    match cat {
        Ok(cat) => Ok(actix_web::web::Json(cat)),
        Err(_) => Err(CatError::CatNotFound)
    }
}

#[post("/cat")]
async fn create_cat(data: web::Json<Cat>, app: web::Data<AppState>) -> Result<Json<String>, CatError> {
    println!("Create Cat {:?}", data);
    let firebase = app.firebase.at("cats");
    let cat = firebase.set::<Cat>(&data).await
    .and_then(|entries| { 
        Ok(entries.data)
    });
    println!("{:?}", cat);
    match cat {
        Ok(cat) => Ok(actix_web::web::Json(cat)),
        Err(_) => Err(CatError::CatCreationFailure)
    }
}

#[put("/cat/{id}")]
async fn update_cat(data: web::Json<Cat>, path: web::Path<String>, app: web::Data<AppState>) -> Result<Json<String>, CatError>  {
    let id = path.into_inner();
    println!("Update Cat {:?}", data);
    let firebase = app.firebase.at("cats").at(&id);
    let cat = firebase.update::<Cat>(&data).await
    .and_then(|entries| { 
        Ok(entries.data)
    });
    println!("{:?}", cat);
    match cat {
        Ok(cat) => Ok(actix_web::web::Json(cat)),
        Err(_) => Err(CatError::CatUpdateFailure)
    }
}

#[delete("/cat/{id}")]
async fn delete_cat(path: web::Path<String>, app: web::Data<AppState>) -> Result<HttpResponse, CatError> {
    let id = path.into_inner();
    println!("Delete Cat {:?}", id);
    let firebase = app.firebase.at("cats").at(&id);
    let result = firebase.delete().await
    .and_then(|entries| { 
        Ok(entries.data)
    });
    println!("{:?}", result);
    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(CatError::CatNotFound)
    }
}