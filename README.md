# Axum Workshop

## What should the app do?
The app handles a list of notes contained in a vector of strings as its state.

Users shall now be able to add simple strings into the list, get or delete them by their index in the list.

## What is your task?
You should use your knowledge gained during the workshop to implement the missing sections of this app.
This includes:
- Logging
- Reading the host and port through an environment variable (localhost and i.e. port 3000)
- Complete the four calls inside the `src/services/handlers.rs` file
- place the JSON payload representations into the `src/services/payloads.rs` file as structs, which derive from serde's `Serialize` and `Deserialize`

TODOs are placed everywhere, where your code is needed, in addition with some constraints the app should comply to.

A possible solution is contained inside the `solution` branch for reference.
