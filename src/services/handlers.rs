use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::{error, info};

use crate::AppState;

// Import our JSON representations
use super::payloads::{
    CreateRequestPayload, CreateResponsePayload, DeleteOneResponsePayload, GetAllReponsePayload,
    GetOneResponsePayload,
};

// TODO: return all currently exisiting notes
// Reponse body JSON: { notes: ["some note", "some other note"] }
//
// Add a logging span to this function
#[tracing::instrument]
pub async fn get_all(State(state): State<AppState>) -> Response {
    // Get our state and lock it, since it uses a mutex
    let notes = state.notes.lock().await;
    // Log a successful response
    info!("Get all notes: {:?}", notes.clone());
    // Create our response by using a tuple containing the statuscode and the payload
    (
        StatusCode::OK,
        Json(GetAllReponsePayload {
            notes: notes.clone(),
        }),
    )
        .into_response()
}

// TODO: create a note
// Request body JSON: { note: "some note" }
// Response body JSON: { addedNote: "some note" }
//
// Add a logging span to this function
#[tracing::instrument]
pub async fn create(
    // extract the state and the payload of the request
    // IMPORTANT: extracting the body must be last one to extract, when path or query extractors
    // are being used simultaneously
    State(state): State<AppState>,
    payload: Json<CreateRequestPayload>,
) -> Response {
    // Get the state as mut, to mutate it
    let mut notes = state.notes.lock().await;
    let note = &payload.note;
    notes.push(note.to_string());
    info!("Create note {}", note);
    (
        StatusCode::OK,
        Json(CreateResponsePayload {
            added_note: note.to_string(),
        }),
    )
        .into_response()
}

// TODO: get one note by index
// Use Path parameter for the index
// Response body JSON: { foundNote: "some note" }
// Errors:
//  index not found: 404 NOT FOUND
//  index invalid: 400 BAD REQUEST
// Help: Use the parse() method of strings to parse the index
//
// Add a logging span to this function
#[tracing::instrument]
pub async fn get_one(Path(index): Path<String>, State(state): State<AppState>) -> Response {
    // Parse a number out of the string
    let index_num: i32 = match index.parse::<i32>() {
        // We ignore the actual error value by using the _
        Err(_) => {
            error!("Index {} is not an integer", index);
            return (StatusCode::BAD_REQUEST, "Index is not an integer").into_response();
        }
        Ok(number) => number,
    };
    if index_num < 0 {
        // Log an error message
        error!("Index {} cannot be negative", index);
        return (StatusCode::BAD_REQUEST, "Index cannot be negative").into_response();
    }
    // We need to convert the index to the usize type, since the get() method of the vector needs
    // this type instead of i32
    let index_usize = usize::try_from(index_num).ok().unwrap();
    let notes = state.notes.lock().await;
    // Match whether we found the note or not
    let found_note = match notes.get(index_usize) {
        None => {
            error!("Note with index {} not found", index_usize);
            return (StatusCode::NOT_FOUND, "Note not found").into_response();
        }
        Some(note) => note.to_string(),
    };
    info!("Get one note: {}", found_note);
    // Shorthand init, since the variable and the struct's field name match
    (StatusCode::OK, Json(GetOneResponsePayload { found_note })).into_response()
}

// TODO: delete note by the given index
// Use Query Parameter to read the index
// Response body JSON: { deletedNote: "some note" }
// Errors:
//  index not found: 404 NOT FOUND
//  index invalid: 400 BAD REQUEST
//
// Add a logging span to this function
#[tracing::instrument]
pub async fn delete_one(
    // Query params is a hashmap
    Query(query_params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Response {
    let index = query_params.get("index");
    // Check if index query param exists
    // Options provide neat methods for checking
    if index.is_none() {
        error!("\"index\" query param must be set");
        return (StatusCode::NOT_FOUND, "index query param must be set").into_response();
    }
    let mut notes = state.notes.lock().await;
    let index_num: i32 = match index.unwrap().parse::<i32>() {
        // We ignore the actual error value by using the _
        Err(_) => {
            error!("Index {} is not an integer", index.unwrap());
            return (StatusCode::BAD_REQUEST, "Index is not an integer").into_response();
        }
        Ok(number) => number,
    };
    // We again need a usize
    let index_usize = match usize::try_from(index_num) {
        Err(_) => {
            error!("Index {} cannot be negative", index.unwrap());
            return (StatusCode::BAD_REQUEST, "Index cannot be negative").into_response();
        }
        Ok(index) => index,
    };
    if index_usize >= notes.len() {
        error!("Note with index {} not found", index_usize);
        return (StatusCode::NOT_FOUND, "Note not found").into_response();
    }
    let found_note = notes.remove(index_usize);
    info!("Delete note: {}", found_note);
    (
        StatusCode::OK,
        Json(DeleteOneResponsePayload {
            deleted_note: found_note,
        }),
    )
        .into_response()
}
