use mongodb::bson;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct Account {
    #[serde(rename = "_id",skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    holder_name:String,
    status: Status
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum Status {
    ACTIVE,
    INACTIVE
}