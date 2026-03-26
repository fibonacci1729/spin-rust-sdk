use spin_sdk::http::{IntoResponse, Request, EmptyBody, body::IncomingBodyExt};
use spin_sdk::key_value::Store;
use spin_sdk::http_service;

#[http_service]
async fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Open the default key-value store
    let store = Store::open_default().await?;

    match *req.method() {
        http::Method::POST => {
            // Add the request (URI, body) tuple to the store
            let key = req.uri().path().to_string();
            let bytes = req.into_body().bytes().await?;
            store.set(key, bytes.to_vec()).await?;
            Ok((http::StatusCode::OK, EmptyBody::new()))
        }
        http::Method::GET => {
            // // Get the value associated with the request URI, or return a 404 if it's not present
            // match store.get(req.uri().path())? {
            //     Some(value) => (StatusCode::OK, Some(value)),
            //     None => (StatusCode::NOT_FOUND, None),
            // }
            todo!()
        }
        http::Method::DELETE => {
            // Delete the value associated with the request URI, if present
            store.delete(req.uri().path().to_string()).await?;
            Ok((http::StatusCode::OK, EmptyBody::new()))
        }
        http::Method::HEAD => {
            // Like GET, except do not return the value
            let key= req.uri().path().to_string();
            let code = if store.exists(key).await? {
                http::StatusCode::OK
            } else {
                http::StatusCode::NOT_FOUND
            };
            Ok((code, EmptyBody::new()))
        }
        // No other methods are currently supported
        _ => Ok((http::StatusCode::METHOD_NOT_ALLOWED, EmptyBody::new())),
    }
}
