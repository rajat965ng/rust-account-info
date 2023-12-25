use std::str::FromStr;

use actix_web::web::Data;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use mongodb::results::InsertOneResult;

use crate::model::Account;

pub async fn find_all(database: &mut Data<Database>) -> Vec<Account> {
    let mut accounts = vec![];
    let collection = database.collection::<Account>("info");
    let mut result =  collection.find(None, None).await.unwrap();
    while result.advance().await.unwrap() {
        accounts.push(result.deserialize_current().unwrap());
    }
    accounts
}

pub async fn save(database: &mut Data<Database>, account: &Account) -> InsertOneResult {
    let collection = database.collection::<Account>("info");
    collection.insert_one(account, None).await.unwrap()
}

pub async fn delete(database: &mut Data<Database>, oid: &String) -> bool {
    let collection = database.collection::<Account>("info");
    let oid = ObjectId::from_str(oid.as_str()).unwrap();
    if collection.delete_one(doc! {"_id":oid}, None).await.unwrap().deleted_count>0 {true} else {false}
}