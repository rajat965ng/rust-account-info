use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct Account {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    holder_name:String,
    status: Status
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum Status {
    ACTIVE,
    INACTIVE
}