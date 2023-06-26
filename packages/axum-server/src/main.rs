use std::io;

use axum::{
    error_handling::HandleError,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    Router, Server,
};
use silkenweb::{dom::Dry, router, task};
use ssr_example_app::app;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let pkg_service = HandleError::new(ServeDir::new("../axum-client/pkg"), io_error_to_response);
    let app = Router::new()
        .nest_service("/pkg", pkg_service)
        .fallback(handler);
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn io_error_to_response(err: io::Error) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, err.to_string())
}

async fn handler(uri: Uri) -> impl IntoResponse {
    let (title, body) = app::<Dry>();
    router::set_url_path(uri.path());
    // I think this is OK, as we only run futures until they're stalled. Axum only
    // supports `Send` handers, so we can't use `task::render_now().await;`. We're
    // relying on `app()` and this to be running on the same thread, which is also
    // OK as there are no `await`s between them.
    task::server::render_now_sync();

    let page_html = format!(
        include_str!("../../app/page.tmpl.html"),
        title_html = title.freeze(),
        body_html = body.freeze(),
        init_script = r#"
                import init, {js_main} from '/pkg/ssr_example_axum_client.js';
                init().then(js_main);
        "#
    );

    Response::builder()
        .status(StatusCode::OK)
        .body(page_html)
        .unwrap()
}
