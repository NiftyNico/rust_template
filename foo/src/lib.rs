use axum::{
    response::Html,
    routing::get,
    Router,
};

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello from Axum + Bazel!</h1>")
}

pub fn create_app() -> Router {
    Router::new().route("/", get(hello_handler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_app() {
        let _app = create_app();
        // Basic test to ensure the app can be created
        assert!(true);
    }
}
