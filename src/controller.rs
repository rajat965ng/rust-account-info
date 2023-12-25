use actix_web::{get, HttpResponse, web};
use actix_web::web::{Data, ServiceConfig};
use mongodb::Database;

use crate::model::Account;


#[get("")]
async fn get_all_accounts(database:Data<Database>) -> HttpResponse {
    let mut accounts = vec![];
    let collection = database.collection::<Account>("info");
    let mut result =  collection.find(None, None).await.unwrap();
    while result.advance().await.unwrap() {
        accounts.push(result.deserialize_current().unwrap());
    }
    HttpResponse::Ok().json(accounts)
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = web::scope("/v1/accounts")
        .service(get_all_accounts);

    conf.service(scope);
}