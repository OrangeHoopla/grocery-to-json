

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}};
    use axum_test::TestServer;
    use crate::api::central::get_routes;


    /// Helper function to create a GET request for a given URI.
    fn send_get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn first() {

        let app = get_routes();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;

        response.assert_status(StatusCode::OK);
        response.assert_text("hello");
    }

}
