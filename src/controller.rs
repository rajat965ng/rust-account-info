use actix_web::{delete, get, HttpResponse, HttpResponseBuilder, post, web};
use actix_web::web::{BytesMut, Data, Path, Payload, ServiceConfig};
use mongodb::Database;

use crate::model::Account;
use crate::service;

#[get("")]
async fn get_all_accounts(mut database:Data<Database>) -> HttpResponse {
    let accounts =  service::find_all(&mut database).await;
    HttpResponse::Ok().json(accounts)
}

#[post("")]
async fn save_accounts(mut database:Data<Database>,payload:Payload) -> HttpResponse {
    let mut body = BytesMut::new();
    let bytes = payload.to_bytes().await.unwrap();
    body.extend_from_slice(&bytes);
    let result = service::save(&mut database,&(serde_json::from_slice::<Account>(&body).unwrap())).await.inserted_id;
    HttpResponse::Created().json(result)
}

#[delete("/{oid}")]
async fn delete_account(mut database:Data<Database>,path:Path<String>) -> HttpResponseBuilder {
    let oid= path.into_inner();
    let is_deleted = service::delete(&mut database,&oid).await;
    if is_deleted {HttpResponse::NoContent()} else {HttpResponse::NotFound()}
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = web::scope("/v1/accounts")
        .service(get_all_accounts)
        .service(save_accounts)
        .service(delete_account);

    conf.service(scope);
}