use axum::{
    body::boxed,
    extract::Multipart,
    http::StatusCode,
    response::Response,
    routing::{on, MethodFilter},
    Router,
};

use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    // Create a new router
    let router = Router::new()
        // Define a route that matches POST requests to the "/upload" path
        .route("/upload", on(MethodFilter::POST, upload));

    // Define the address that the server will bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create a new server, bind it to the address, serve the router, and await indefinitely
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

// The upload function handles file uploads
pub async fn upload(mut multipart: Multipart) -> Result<Response, StatusCode> {
    // Iterate over each field in the multipart form data
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        // Get the filename of the current field, or skip to the next field if it doesn't have one
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        // Create a new file with the same name as the uploaded file, but with a
        // prefix
        let mut file = tokio::fs::File::create(format!("./uploaded-{}", filename))
            .await
            .unwrap();

        // Write each chunk of data from the uploaded file to the new file
        while let Some(chunk) = field.chunk().await.unwrap() {
            file.write_all(&chunk).await.unwrap();
        }

        // Return a response with a 201 Created status code and a body of "OK"
        return Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(boxed("OK".to_string()))
            .unwrap());
    }

    // If something goes wrong, return a response with a 500 Internal Server Error status code
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
