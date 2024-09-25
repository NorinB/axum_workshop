use serde::{Deserialize, Serialize};

// TODO: add request and response bodies here with rust's structs and serde
// Request bodies, so only Deserialize
// We need to derive Debug to be able to log it
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequestPayload {
    pub note: String,
}

// Response bodies, so only Serialize
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllReponsePayload {
    pub notes: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponsePayload {
    pub added_note: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOneResponsePayload {
    pub found_note: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOneResponsePayload {
    pub deleted_note: String,
}
