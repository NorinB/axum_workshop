use axum::response::Response;

// TODO: return all currently exisiting notes
// Reponse body JSON: { notes: ["some note", "some other note"] }
pub async fn get_all() -> Response {
    todo!();
}

// TODO: create a note
// Request body JSON: { note: "some note" }
// Response body JSON: { addedNote: "some note" }
pub async fn create() -> Response {
    todo!();
}

// TODO: get one note by index
// Use Path parameter for the index
// Response body JSON: { foundNote: "some note" }
// Errors:
//  index not found: 404 NOT FOUND
//  index invalid: 400 BAD REQUEST
// Help: Use the parse() method of strings to parse the index
pub async fn get_one() -> Response {
    todo!();
}

// TODO: delete note by the given index
// Use Query Parameter to read the index
// Response body JSON: { deletedNote: "some note" }
// Errors:
//  index not found: 404 NOT FOUND
//  index invalid: 400 BAD REQUEST
pub async fn delete_one() -> Response {
    todo!();
}
